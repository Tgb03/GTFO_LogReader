use std::collections::HashMap;

use glr_core::{time::Time, token::Token};
use serde::Serialize;

use crate::{core::token_parser::TokenParser, dll_exports::{structs::CallbackInfo, token_parsers::TokenParserInner}, output_trait::OutputTrait};

#[derive(Default)]
pub struct CallbackWrapper<P>
where 
    P: TokenParserInner {

    callbacks: HashMap<u32, CallbackInfo>,
    token_parser: P,

}

impl<P: TokenParserInner> CallbackWrapper<P> {

    pub fn add_callback(&mut self, callback: CallbackInfo) {
        self.callbacks.insert(callback.get_id(), callback);
    }

    pub fn remove_callback(&mut self, callback_id: u32) {
        self.callbacks.remove(&callback_id);
    }

}

impl<P> CallbackWrapper<P>
where 
    P: TokenParserInner + Default {
    
    pub fn reset_token_parser(&mut self) {
        self.token_parser = P::default();
    }
}

impl<P: TokenParserInner> TokenParser for CallbackWrapper<P> {
    fn parse_token(&mut self, time: Time, token: &Token) {
        self.token_parser.parse(time, token, &self.callbacks);
    }
}

impl<O: Serialize> OutputTrait<O> for HashMap<u32, CallbackInfo> {
    fn output(&self, data: O) {
        for callback in self.values() {
            if let Some(event) = callback.get_event_callback() {
                let converter = callback.get_message_type();
                let context = callback.get_context();
                if let Some(result_string) = converter.convert(&data) {
                    event(context.get_ptr(), result_string.as_ptr())
                }
            }
        }
    }
}
