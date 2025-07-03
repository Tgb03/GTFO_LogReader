use std::{
    ffi::{CStr, c_char},
    os::raw::c_void,
    path::PathBuf,
    sync::OnceLock,
};

use crate::dll_exports::structs::{CallbackInfo, MainThread};

pub type EventCallback = extern "C" fn(message: *const c_char);
static MAIN_THREAD: OnceLock<MainThread> = OnceLock::new();

/// starts a folder listener in that file_path. This file_path must
/// containg GTFO logs that the program will then read and output
/// all the information it gets.
///
/// In code this creates an instance of a FolderWatcher
/// if you call this function again prior to deleting
/// said FolderWatcher it will shutdown the previous on and
/// all threads connected to it and then start a new one.
#[unsafe(no_mangle)]
pub extern "C" fn start_listener(file_path: *const c_char) {
    let path = unsafe {
        if file_path.is_null() {
            return;
        }

        let c_str = CStr::from_ptr(file_path);
        let string = c_str.to_string_lossy();

        PathBuf::from(&*string)
    };

    let thread = MAIN_THREAD.get_or_init(|| MainThread::create(path.clone()));
    thread.change_logs_folder(path);
}

///
#[unsafe(no_mangle)]
pub extern "C" fn add_callback(
    code: u8,
    message_type: u8,
    channel_id: u32,
    event_callback_ptr: *const c_void,
) {
    let code = code.into();
    let message_type = message_type.into();
    let event_callback = if event_callback_ptr.is_null() {
        None
    } else {
        Some(unsafe {
            // Cast the void pointer into a function pointer
            std::mem::transmute::<*const c_void, EventCallback>(event_callback_ptr)
        })
    };

    let callback_info = CallbackInfo::new(code, message_type, channel_id, event_callback);
    MAIN_THREAD
        .get()
        .map(|t| t.register_callback(callback_info));
}

///
#[unsafe(no_mangle)]
pub extern "C" fn remove_callback(channel_id: u32) {
    MAIN_THREAD.get().map(|t| t.remove_callback(channel_id));
}
