use glr_core::{time::Time, token::Token};
use serde::Serialize;

use crate::output_trait::OutputTrait;


pub mod token_parser_base;
pub mod token_parser_locations;
pub mod token_parser_runs;
pub mod token_parser_seeds;

pub trait TokenParserInner {
    type Output: Serialize;

    fn parse(&mut self, time: Time, token: &Token, callback_handler: &impl OutputTrait<Self::Output>);
}
