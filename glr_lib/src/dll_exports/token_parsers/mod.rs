use glr_core::{time::Time, token::Token};
use serde::Serialize;

use crate::output_trait::OutputTrait;


pub mod token_parser_base;
pub mod token_parser_locations;
pub mod token_parser_runs;
pub mod token_parser_seeds;

pub trait TokenParserInner {
    type Output: Serialize;

    fn parse(&mut self, time: Time, token: &Token, callback_handler: &mut impl OutputTrait<Self::Output>);
    fn parse_tokens(
        &mut self, 
        tok_iter: impl Iterator<Item = (Time, Token)>, 
        callback_handler: &mut impl OutputTrait<Self::Output>
    ) {
        for (time, token) in tok_iter {
            self.parse(time, &token, callback_handler);
        }
    }
}
