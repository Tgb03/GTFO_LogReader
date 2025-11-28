use std::{
    ffi::c_char,
    os::raw::c_void,
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
    time::Duration,
};

use glr_core::{time::Time, token::Token};
use might_sleep::prelude::CpuLimiter;

use crate::{
    core::{
        token_parser::{IterTokenParser, TokenParser},
        tokenizer::{GenericTokenizer, TokenizeIter, TokenizerGetIter},
    },
    dll_exports::{
        callback_handler::{CallbackClone, HasCallbackHandler},
        enums::{SubscribeCode, SubscriptionType},
        token_parsers::{
            CallbackTokenParser, token_parser_base::TokenParserBase,
            token_parser_locations::TokenParserLocations, token_parser_runs::TokenParserRuns,
            token_parser_seeds::TokenParserSeed,
        },
    },
    readers::{file_reader::FileReader, folder_watcher::FolderWatcher},
};

pub type EventCallback = extern "C" fn(context: *const c_void, message: *const c_char);

#[derive(Debug, Clone, Copy)]
pub struct ThreadSafePtr {
    p: *const c_void,
}

impl<T> From<*const T> for ThreadSafePtr {
    fn from(value: *const T) -> Self {
        Self {
            p: value as *const c_void,
        }
    }
}

impl ThreadSafePtr {
    pub fn get_ptr(&self) -> *const c_void {
        self.p
    }
}

unsafe impl Send for ThreadSafePtr {}
unsafe impl Sync for ThreadSafePtr {}

#[derive(Clone)]
pub struct CallbackInfo {
    code: SubscribeCode,
    message_type: SubscriptionType,
    channel_id: u32,

    context: ThreadSafePtr,
    event_callback: Option<EventCallback>,
}

impl CallbackInfo {
    pub fn new(
        code: SubscribeCode,
        message_type: SubscriptionType,
        channel_id: u32,
        context: ThreadSafePtr,
        event_callback: Option<EventCallback>,
    ) -> CallbackInfo {
        CallbackInfo {
            code,
            message_type,
            channel_id,
            context,
            event_callback,
        }
    }

    pub fn get_code(&self) -> SubscribeCode {
        self.code
    }

    pub fn get_message_type(&self) -> SubscriptionType {
        self.message_type
    }

    pub fn get_id(&self) -> u32 {
        self.channel_id
    }

    pub fn get_event_callback(&self) -> &Option<EventCallback> {
        &self.event_callback
    }

    pub fn get_context(&self) -> &ThreadSafePtr {
        &self.context
    }
}

pub struct MainThread {
    folder_watcher: FolderWatcher,
    send_callbacks: Sender<CallbackInfo>,

    shutdown: Sender<()>,
    join: Option<JoinHandle<()>>,
}

impl MainThread {
    pub fn create(folder_path: Option<PathBuf>) -> MainThread {
        let (recv, folder_watcher) = FolderWatcher::new_watcher(folder_path);
        let (shutdown_sender, shutdown_recv) = mpsc::channel::<()>();
        let (callback_sender, callback_recv) = mpsc::channel::<CallbackInfo>();
        let file_reader = FileReader::new(recv);

        let join = thread::spawn(|| Self::thread_run(file_reader, callback_recv, shutdown_recv));

        MainThread {
            folder_watcher,
            send_callbacks: callback_sender,
            shutdown: shutdown_sender,
            join: Some(join),
        }
    }

    pub fn register_callback(&self, callback: CallbackInfo) {
        let _ = self.send_callbacks.send(callback);
    }

    pub fn remove_callback(&self, code: SubscribeCode, id: u32) {
        let _ = self.send_callbacks.send(CallbackInfo {
            channel_id: id,
            code,
            context: ThreadSafePtr {
                p: 0 as *mut c_void,
            },
            event_callback: None,
            message_type: 0.into(),
        });
    }

    pub fn change_logs_folder(&self, new_path: PathBuf) {
        self.folder_watcher.update_path(new_path);
    }

    pub fn static_run(mut paths: Vec<PathBuf>, callback: CallbackInfo) {
        let tokenizer = GenericTokenizer::all_tokenizers();

        while let Some(path) = paths.pop() {
            let mut parser: Box<dyn CallbackTokenParser> = match callback.code {
                SubscribeCode::Tokenizer => Box::new(TokenParserBase::default()),
                SubscribeCode::RunInfo => Box::new(TokenParserRuns::default()),
                SubscribeCode::Mapper => Box::new(TokenParserLocations::default()),
                SubscribeCode::SeedIndexer => Box::new(TokenParserSeed::default()),
            };

            parser.add_callback(callback.clone());

            let Some(text) = FileReader::static_read(path.clone()) else {
                println!("Could not read path: {:?}", path);
                continue;
            };

            let tok_iter = TokenizeIter::new(text.split("\n"), &tokenizer);

            parser.parse_tokens(tok_iter);
            if let Some(last_line) = text.split("\n").last().map(|v| Time::from(v)).flatten() {
                parser.parse_token(last_line, &Token::LogFileEnd);
            }
        }
    }

    fn thread_run(
        mut file_reader: FileReader,
        callback_recv: Receiver<CallbackInfo>,
        shutdown: Receiver<()>,
    ) {
        let mut limiter = CpuLimiter::new(Duration::from_millis(200));
        let tokenizer = GenericTokenizer::all_tokenizers();

        let mut parser_base = TokenParserBase::default();
        let mut parser_seeds = TokenParserSeed::default();
        let mut parser_mapper = TokenParserLocations::default();
        let mut parser_runs = TokenParserRuns::default();

        loop {
            if let Ok(()) = shutdown.try_recv() {
                break;
            }

            while let Ok(callback) = callback_recv.try_recv() {
                match callback.code {
                    SubscribeCode::Tokenizer => parser_base.add_callback(callback),
                    SubscribeCode::RunInfo => parser_runs.add_callback(callback),
                    SubscribeCode::Mapper => parser_mapper.add_callback(callback),
                    SubscribeCode::SeedIndexer => parser_seeds.add_callback(callback),
                }
            }

            if file_reader.get_was_new_file() {
                parser_base = parser_base.clone_callbacks();
                parser_seeds = parser_seeds.clone_callbacks();
                parser_mapper = parser_mapper.clone_callbacks();
                parser_runs = parser_runs.clone_callbacks();
            }

            if let Some(new_lines) = file_reader.get_new_lines() {
                tokenizer
                    .tokenize_to_iter(&new_lines)
                    .for_each(|(time, token)| {
                        parser_base.parse_token(time, &token);
                        parser_mapper.parse_token(time, &token);
                        parser_seeds.parse_token(time, &token);
                        parser_runs.parse_token(time, &token);
                    });
            }

            limiter.might_sleep();
        }
    }
}

impl Drop for MainThread {
    fn drop(&mut self) {
        let _ = self.shutdown.send(());
        let _ = match self.join.take() {
            Some(jh) => {
                let _ = jh.join();
            }
            None => {}
        };
    }
}
