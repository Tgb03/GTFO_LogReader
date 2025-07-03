use serde::{Deserialize, Serialize};

use crate::{
    dll_exports::callback_handler::HasCallbackHandler,
    output_trait::{OutputSeedIndexer, OutputTrait},
    seed_gen::consumers::base_consumer::Consumer,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OutputSeed;

impl<O> Consumer<O> for OutputSeed
where
    O: HasCallbackHandler,
{
    fn take(&mut self, seed: f32, output: &mut O) -> bool {
        output.output(OutputSeedIndexer::Seed(seed));

        true
    }
}
