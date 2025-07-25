use std::{
    ffi::{c_char, CStr},
    os::raw::c_void,
    path::PathBuf,
    sync::{Mutex, OnceLock},
};

use crate::dll_exports::structs::{CallbackInfo, MainThread};

pub type EventCallback = extern "C" fn(context: *const c_void, message: *const c_char);
static MAIN_THREAD: OnceLock<Mutex<Option<MainThread>>> = OnceLock::new();

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

    let _ = MAIN_THREAD.get_or_init(|| Some(MainThread::create(None)).into())
        .lock()
        .map(|mut v| 
            match v.as_mut() {
                Some(m_th) => m_th.change_logs_folder(path),
                None => {
                    *v = Some(MainThread::create(Some(path)));
                },
            }
        );
}

///
#[unsafe(no_mangle)]
pub extern "C" fn add_callback(
    code: u8,
    message_type: u8,
    channel_id: u32,
    context: *const c_void,
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

    let callback_info = CallbackInfo::new(code, message_type, channel_id, context.into(), event_callback);
    let _ = MAIN_THREAD
        .get_or_init(|| Some(MainThread::create(None)).into())
        .lock()
        .map(|mut v| 
            v.as_mut().map(|v| v.register_callback(callback_info))
        );
}

///
#[unsafe(no_mangle)]
pub extern "C" fn remove_callback(code: u8, channel_id: u32) {
    let code = code.into();

    let _ = MAIN_THREAD
        .get_or_init(|| Some(MainThread::create(None)).into())
        .lock()
        .map(|mut v| 
            v.as_mut().map(|v| v.remove_callback(code, channel_id))
        );
}

#[unsafe(no_mangle)]
pub extern "C" fn process_paths(
    paths: *const *const c_char, 
    len: u32,
    code: u8,
    message_type: u8,
    context: *const c_void,
    event_callback_ptr: *const c_void,
) {
    if paths.is_null() {
        return;
    }

    let slice = unsafe { std::slice::from_raw_parts(paths, len as usize) };

    let pathbufs: Vec<PathBuf> = slice
        .iter()
        .filter_map(|&ptr| {
            if ptr.is_null() {
                None
            } else {
                let c_str = unsafe { CStr::from_ptr(ptr) };
                Some(PathBuf::from(c_str.to_string_lossy().into_owned()))
            }
        })
        .collect();
    
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

    let callback_info = CallbackInfo::new(code, message_type, 0, context.into(), event_callback);

    MainThread::static_run(pathbufs, callback_info);
}

#[unsafe(no_mangle)]
pub extern "C" fn shutdown_all() {
    MAIN_THREAD.get()
        .map(|v| v.lock().ok())
        .flatten()
        .as_mut()
        .map(|v| v.take());      
}
