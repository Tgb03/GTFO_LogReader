use std::collections::HashMap;

use crate::{core::{time::Time, token::Token, token_parser::TokenParser}, dll_exports::{callback_handler::HasCallbackHandler, structs::CallbackInfo}, output_trait::OutputTrait, run_gen::{run_generator::RunGenerator, split::NamedSplit}};

#[derive(Default)]
pub struct TokenParserRuns {
    
    callback_handler: HashMap<u32, CallbackInfo>,

    run_parser: RunGenerator<NamedSplit>,

}

impl HasCallbackHandler for TokenParserRuns {
    fn get_callback_handler(&self) -> &HashMap<u32, CallbackInfo> {
        &self.callback_handler
    }

    fn get_callback_handler_mut(&mut self) -> &mut HashMap<u32, CallbackInfo> {
        &mut self.callback_handler
    }
}

impl TokenParser for TokenParserRuns {
    fn parse_token(&mut self, time: Time, token: Token) {
        if self.callback_handler.is_empty() {
            return;
        }
        
        if let Some(res) = self.run_parser.accept_token(time, &token) {
            self.output(res);
        }
    }
}
