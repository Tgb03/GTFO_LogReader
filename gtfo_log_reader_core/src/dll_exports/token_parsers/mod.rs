use crate::{core::token_parser::TokenParser, dll_exports::callback_handler::HasCallbackHandler};

pub mod token_parser_base;
pub mod token_parser_seeds;
pub mod token_parser_locations;
pub mod token_parser_runs;

pub trait CallbackTokenParser: HasCallbackHandler + TokenParser {}

impl<T> CallbackTokenParser for T
where
    T: HasCallbackHandler + TokenParser {}
