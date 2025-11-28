use std::{
    path::PathBuf,
    sync::{Mutex, OnceLock},
};

use crate::dll_exports::{
    enums::SubscribeCode,
    structs::{CallbackInfo, MainThread},
};

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
    MainThread::static_run(paths, callback);
}
