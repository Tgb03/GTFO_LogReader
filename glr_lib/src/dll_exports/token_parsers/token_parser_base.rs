
use glr_core::{time::Time, token::Token};

use crate::{
    dll_exports::token_parsers::TokenParserInner, output_trait::OutputTrait
};

#[derive(Default)]
pub struct TokenParserBase;


impl TokenParserInner for TokenParserBase {
    fn parse(&mut self, _: Time, token: &Token, callback_handler: &impl OutputTrait<Token>) {
        callback_handler.output(token.clone());
    }
    
    type Output = Token;
}
