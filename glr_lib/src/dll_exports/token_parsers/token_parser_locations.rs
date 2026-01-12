use glr_core::{location::Location, time::Time, token::Token};

use crate::{
    dll_exports::token_parsers::TokenParserInner, mapper::{
        key_generator::KeyGenerator, location_generator::LocationGenerator,
        objective_item_generator::ObjectiveItemGenerator,
    }, output_trait::OutputTrait,
};

#[derive(Default)]
pub struct TokenParserLocations {
    key_gen: KeyGenerator,
    obj_gen: ObjectiveItemGenerator,
}

impl TokenParserInner for TokenParserLocations {
    type Output = Location;

    fn parse(&mut self, _: Time, token: &Token, callback_handler: &impl OutputTrait<Location>) {
        if let Some(key) = self.key_gen.accept_token(&token) {
            callback_handler.output(key)
        }

        if let Some(obj) = self.obj_gen.accept_token(&token) {
            callback_handler.output(obj)
        }
    }
    
}
