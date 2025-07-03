use std::collections::HashMap;

use crate::{
    core::{time::Time, token::Token, token_parser::TokenParser},
    dll_exports::{callback_handler::HasCallbackHandler, structs::CallbackInfo},
    output_trait::OutputTrait,
};

#[derive(Default)]
pub struct TokenParserBase {
    callback_handler: HashMap<u32, CallbackInfo>,
}

impl HasCallbackHandler for TokenParserBase {
    fn get_callback_handler(&self) -> &HashMap<u32, CallbackInfo> {
        &self.callback_handler
    }

    fn get_callback_handler_mut(&mut self) -> &mut HashMap<u32, CallbackInfo> {
        &mut self.callback_handler
    }
}

impl TokenParser for TokenParserBase {
    fn parse_token(&mut self, _: Time, token: Token) {
        if self.callback_handler.is_empty() {
            return;
        }

        self.output(token);
    }
}
