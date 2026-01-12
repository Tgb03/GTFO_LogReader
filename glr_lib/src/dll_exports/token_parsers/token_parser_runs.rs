
use glr_core::{run_gen_result::RunGeneratorResult, split::NamedSplit, time::Time, token::Token};

use crate::{
    dll_exports::token_parsers::TokenParserInner, output_trait::OutputTrait, run_gen::run_generator::RunGenerator
};

#[derive(Default)]
pub struct TokenParserRuns {
    run_parser: RunGenerator<NamedSplit>,
}

impl TokenParserInner for TokenParserRuns {
    type Output = RunGeneratorResult;

    fn parse(&mut self, time: Time, token: &Token, callback_handler: &impl OutputTrait<RunGeneratorResult>) {
        if let Some(res) = self.run_parser.accept_token(time, &token) {
            callback_handler.output(res)
        }
    }
}
