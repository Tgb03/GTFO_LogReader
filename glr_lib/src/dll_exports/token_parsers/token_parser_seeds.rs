use std::collections::HashMap;

use glr_core::{seed_indexer_result::OutputSeedIndexer, time::Time, token::Token};

use crate::{
    core::token_parser::TokenParser, dll_exports::{callback_handler::HasCallbackHandler, structs::CallbackInfo}, seed_gen::levels::LevelDescriptors, output_trait::OutputTrait, seed_gen::{consumers::base_consumer::Consumer, unity_random::UnityRandom}
};

#[derive(Default)]
pub struct TokenParserSeed {
    callback_handler: HashMap<u32, CallbackInfo>,
    level_descriptors: LevelDescriptors,
}

impl HasCallbackHandler for TokenParserSeed {
    fn get_callback_handler(&self) -> &HashMap<u32, CallbackInfo> {
        &self.callback_handler
    }

    fn get_callback_handler_mut(&mut self) -> &mut HashMap<u32, CallbackInfo> {
        &mut self.callback_handler
    }
}

impl TokenParser for TokenParserSeed {
    fn parse_token(&mut self, _: Time, token: Token) {
        if self.callback_handler.is_empty() {
            return;
        }

        if let Token::SelectExpedition(level, seed) = token {
            self.output(OutputSeedIndexer::GenerationStart);

            let mut unity_random = UnityRandom::from(seed);
            self.level_descriptors
                .get_level(&level)
                .map(|v| v.take(&mut unity_random, &mut self.callback_handler));
            
            self.output(OutputSeedIndexer::GenerationEnd);
        }
    }
}
