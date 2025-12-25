use std::{
    ffi::{c_char, c_void, CStr},
    path::PathBuf,
};

use glr_lib::dll_exports::structs::{CallbackInfo, EventCallback};

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

    glr_lib::dll_exports::functions::start_listener(path);
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

    glr_lib::dll_exports::functions::add_callback(CallbackInfo::new(
        code,
        message_type,
        channel_id,
        context.into(),
        event_callback,
    ));
}

///
#[unsafe(no_mangle)]
pub extern "C" fn remove_callback(code: u8, channel_id: u32) {
    let code = code.into();

    glr_lib::dll_exports::functions::remove_callback(code, channel_id);
}

/* 
#[unsafe(no_mangle)]
pub extern "C" fn process_seed(
    seed: i32,
    message_type: u8,
    context: *const c_void,
    event_callback_ptr: *const c_void,
) {
    let code = SubscribeCode::SeedIndexer;
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
}
*/

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

    glr_lib::dll_exports::functions::process_paths(pathbufs, callback_info);
}

#[unsafe(no_mangle)]
pub extern "C" fn shutdown_all() {
    glr_lib::dll_exports::functions::shutdown_all();
}
