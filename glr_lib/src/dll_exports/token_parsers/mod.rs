use std::collections::HashMap;

use crate::{core::token_parser::TokenParser, dll_exports::{callback_handler::HasCallbackHandler, structs::CallbackInfo}};

pub mod token_parser_base;
pub mod token_parser_seeds;
pub mod token_parser_locations;
pub mod token_parser_runs;

pub trait CallbackTokenParser: HasCallbackHandler + TokenParser {}

impl<T> CallbackTokenParser for T
where
    T: HasCallbackHandler + TokenParser {}

impl HasCallbackHandler for HashMap<u32, CallbackInfo> {
    fn get_callback_handler(&self) -> &HashMap<u32, CallbackInfo> {
        self
    }

    fn get_callback_handler_mut(&mut self) -> &mut HashMap<u32, CallbackInfo> {
        self
    }
}
