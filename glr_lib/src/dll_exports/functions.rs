use std::{
    path::PathBuf,
    sync::{Mutex, OnceLock},
};

use glr_core::{data::LevelDescriptor, time::Time, token::Token};

use crate::{core::token_parser::TokenParser, dll_exports::{
    callback_handler::CallbackWrapper, enums::SubscribeCode, structs::{CallbackInfo, MainThread}, token_parsers::{token_parser_base::TokenParserBase, token_parser_locations::TokenParserLocations, token_parser_runs::TokenParserRuns, token_parser_seeds::TokenParserSeed}
}};

static MAIN_THREAD: OnceLock<Mutex<Option<MainThread>>> = OnceLock::new();

pub fn start_listener(file_path: PathBuf) {
    let _ = MAIN_THREAD
        .get_or_init(|| Some(MainThread::create(None)).into())
        .lock()
        .map(|mut v| match v.as_mut() {
            Some(m_th) => m_th.change_logs_folder(file_path),
            None => {
                *v = Some(MainThread::create(Some(file_path)));
            }
        });
}

pub fn add_callback(callback: CallbackInfo) {
    let _ = MAIN_THREAD
        .get_or_init(|| Some(MainThread::create(None)).into())
        .lock()
        .map(|mut v| v.as_mut().map(|v| v.register_callback(callback)));
}

pub fn remove_callback(code: SubscribeCode, channel_id: u32) {
    let _ = MAIN_THREAD
        .get_or_init(|| Some(MainThread::create(None)).into())
        .lock()
        .map(|mut v| v.as_mut().map(|v| v.remove_callback(code, channel_id)));
}

pub fn shutdown_all() {
    MAIN_THREAD
        .get()
        .map(|v| v.lock().ok())
        .flatten()
        .as_mut()
        .map(|v| v.take());
}

pub fn process_paths(paths: Vec<PathBuf>, callback: CallbackInfo) {
    match callback.get_code() {
        SubscribeCode::Tokenizer => MainThread::static_run::<TokenParserBase>(paths, callback),
        SubscribeCode::RunInfo => MainThread::static_run::<TokenParserRuns>(paths, callback),
        SubscribeCode::Mapper => MainThread::static_run::<TokenParserLocations>(paths, callback),
        SubscribeCode::SeedIndexer => MainThread::static_run::<TokenParserSeed>(paths, callback),
    }
}

pub fn process_seed(seed: i32, callback: CallbackInfo) {
    let mut parser = CallbackWrapper::<TokenParserSeed>::default();
    
    parser.add_callback(callback);
    parser.parse_token(Time::default(), &Token::SelectExpedition(LevelDescriptor::default(), seed));
}
