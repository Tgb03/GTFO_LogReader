use std::{
    ffi::c_char, os::raw::c_void, path::PathBuf, sync::mpsc::{self, Receiver, Sender}, thread::{self, JoinHandle}, time::Duration
};

use might_sleep::prelude::CpuLimiter;

use crate::{
    core::{
        token_parser::{IterTokenParser, TokenParser}, tokenizer::{AllTokenizer, TokenizeIter, TokenizerGetIter}
    },
    dll_exports::{
        callback_handler::CallbackWrapper, enums::{SubscribeCode, SubscriptionType}, token_parsers::{
            TokenParserInner, token_parser_base::TokenParserBase, token_parser_locations::TokenParserLocations, token_parser_runs::TokenParserRuns, token_parser_seeds::TokenParserSeed
        }
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

    pub fn static_run<TP: TokenParserInner + Default>(mut paths: Vec<PathBuf>, callback: CallbackInfo) {
        let mut parser = CallbackWrapper::<TP>::default();

        parser.add_callback(callback.clone());
        
        while let Some(path) = paths.pop() {
            parser.reset_token_parser();
            
            let Some(text) = FileReader::static_read(path.clone()) else {
                println!("Could not read path: {:?}", path);
                continue;
            };

            let tok_iter = TokenizeIter::new(
                text, 
                AllTokenizer
            );

            parser.parse_tokens(tok_iter);
        }
    }

    fn thread_run(
        mut file_reader: FileReader,
        callback_recv: Receiver<CallbackInfo>,
        shutdown: Receiver<()>,
    ) {
        let mut limiter = CpuLimiter::new(Duration::from_millis(200));
        let tokenizer = AllTokenizer;

        let mut parser_base = CallbackWrapper::<TokenParserBase>::default();
        let mut parser_seeds = CallbackWrapper::<TokenParserRuns>::default();
        let mut parser_mapper = CallbackWrapper::<TokenParserLocations>::default();
        let mut parser_runs = CallbackWrapper::<TokenParserSeed>::default();

        loop {
            if let Ok(()) = shutdown.try_recv() {
                break;
            }

            while let Ok(callback) = callback_recv.try_recv() {
                if callback.event_callback.is_some() {
                    match callback.code {
                        SubscribeCode::Tokenizer => parser_base.add_callback(callback),
                        SubscribeCode::RunInfo => parser_runs.add_callback(callback),
                        SubscribeCode::Mapper => parser_mapper.add_callback(callback),
                        SubscribeCode::SeedIndexer => parser_seeds.add_callback(callback),
                    }
                } else {
                    match callback.code {
                        SubscribeCode::Tokenizer => parser_base.remove_callback(callback.get_id()),
                        SubscribeCode::RunInfo => parser_runs.remove_callback(callback.get_id()),
                        SubscribeCode::Mapper => parser_mapper.remove_callback(callback.get_id()),
                        SubscribeCode::SeedIndexer => parser_seeds.remove_callback(callback.get_id()),
                    }
                }
            }

            if file_reader.get_was_new_file() {
                parser_base.reset_token_parser();
                parser_seeds.reset_token_parser();
                parser_mapper.reset_token_parser();
                parser_runs.reset_token_parser();
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
