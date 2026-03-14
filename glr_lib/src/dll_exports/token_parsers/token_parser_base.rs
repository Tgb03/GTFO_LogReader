
use glr_core::{time::Time, token::Token};

use crate::{
    dll_exports::token_parsers::TokenParserInner, output_trait::OutputTrait
};

#[derive(Default)]
pub struct TokenParserBase;


impl TokenParserInner for TokenParserBase {
    type Output = Token;
    
    fn parse(&mut self, _: Time, token: &Token, callback_handler: &mut impl OutputTrait<Token>) {
        callback_handler.output(token.clone());
    }
}
