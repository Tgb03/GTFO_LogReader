
use glr_core::{seed_indexer_result::OutputSeedIndexer, time::Time, token::Token};

use crate::{
    dll_exports::token_parsers::TokenParserInner, output_trait::OutputTrait, seed_gen::{consumers::base_consumer::Consumer, levels::LevelDescriptors, unity_random::UnityRandom}
};

#[derive(Default)]
pub struct TokenParserSeed {
    level_descriptors: LevelDescriptors,
}

impl TokenParserInner for TokenParserSeed {
    type Output = OutputSeedIndexer;

    fn parse(&mut self, _: Time, token: &Token, callback_handler: &impl OutputTrait<OutputSeedIndexer>) {
        if let Token::SelectExpedition(level, seed) = token {
            callback_handler.output(OutputSeedIndexer::GenerationStart(level.to_string()));

            let mut unity_random = UnityRandom::from(*seed);
            self.level_descriptors
                .get_level(&level)
                .map(|v| v.take(&mut unity_random, callback_handler));

            callback_handler.output(OutputSeedIndexer::GenerationEnd);
        }
    }
}
