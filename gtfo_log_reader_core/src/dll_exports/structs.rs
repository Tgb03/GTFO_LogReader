use std::{
    path::PathBuf, sync::mpsc::{self, Receiver, Sender}, thread::{self, JoinHandle}, time::Duration
};

use might_sleep::prelude::CpuLimiter;

use crate::{
    core::{
        token_parser::IterTokenParser,
        tokenizer::{GenericTokenizer, Tokenizer},
    }, dll_exports::{
        callback_handler::HasCallbackHandler,
        enums::{SubscribeCode, SubscriptionType},
        functions::EventCallback,
        token_parsers::{token_parser_base::TokenParserBase, token_parser_seeds::TokenParserSeed},
    }, readers::{file_reader::FileReader, folder_watcher::FolderWatcher}
};

#[derive(Default)]
pub struct CallbackInfo {
    code: SubscribeCode,
    message_type: SubscriptionType,
    channel_id: u32,

    event_callback: Option<EventCallback>,
}

impl CallbackInfo {
    pub fn new(
        code: SubscribeCode,
        message_type: SubscriptionType,
        channel_id: u32,
        event_callback: Option<EventCallback>,
    ) -> CallbackInfo {
        CallbackInfo {
            code,
            message_type,
            channel_id,
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
            ..Default::default()
        });
    }

    pub fn change_logs_folder(&self, new_path: PathBuf) {
        self.folder_watcher.update_path(new_path);
    }

    fn thread_run(
        mut file_reader: FileReader,
        callback_recv: Receiver<CallbackInfo>,
        shutdown: Receiver<()>,
    ) {
        let mut limiter = CpuLimiter::new(Duration::from_millis(500));
        let tokenizer = GenericTokenizer::all_tokenizers();

        let mut parser_base = TokenParserBase::default();
        let mut parser_seeds = TokenParserSeed::default();

        loop {
            if let Ok(()) = shutdown.try_recv() {
                break;
            }

            if let Ok(callback) = callback_recv.try_recv() {
                match callback.code {
                    SubscribeCode::Tokenizer => parser_base.add_callback(callback),
                    SubscribeCode::RunInfo => panic!("SubscribeCode::RunInfo not implemented"),
                    SubscribeCode::Mapper => panic!("SubscribeCode::Mapper not implemented"),
                    SubscribeCode::SeedIndexer => parser_seeds.add_callback(callback),
                }
            }

            if let Some(new_lines) = file_reader.get_new_lines() {
                let tokens = tokenizer.tokenize(&new_lines);

                parser_base.parse_tokens(tokens.iter().cloned());
                parser_seeds.parse_tokens(tokens.iter().cloned());
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
