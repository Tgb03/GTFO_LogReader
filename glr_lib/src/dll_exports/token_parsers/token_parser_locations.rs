use std::collections::HashMap;

use glr_core::{time::Time, token::Token};

use crate::{core::token_parser::TokenParser, dll_exports::{callback_handler::HasCallbackHandler, structs::CallbackInfo}, mapper::{key_generator::KeyGenerator, location_generator::LocationGenerator, objective_item_generator::ObjectiveItemGenerator}, output_trait::OutputTrait};


#[derive(Default)]
pub struct TokenParserLocations {
    
    callback_handler: HashMap<u32, CallbackInfo>,

    key_gen: KeyGenerator,
    obj_gen: ObjectiveItemGenerator,

}

impl HasCallbackHandler for TokenParserLocations {
    fn get_callback_handler(&self) -> &HashMap<u32, CallbackInfo> {
        &self.callback_handler
    }

    fn get_callback_handler_mut(&mut self) -> &mut HashMap<u32, CallbackInfo> {
        &mut self.callback_handler
    }
}

impl TokenParser for TokenParserLocations {
    fn parse_token(&mut self, _: Time, token: Token) {
        if self.callback_handler.is_empty() {
            return;
        }
        
        if let Some(key) = self.key_gen.accept_token(&token) {
            self.output(key);
        }

        if let Some(obj) = self.obj_gen.accept_token(&token) {
            self.output(obj);
        }
    }
}


