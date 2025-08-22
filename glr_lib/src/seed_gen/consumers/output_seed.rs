
use glr_core::seed_indexer_result::OutputSeedIndexer;
use serde::{Deserialize, Serialize};

use crate::{
    dll_exports::callback_handler::HasCallbackHandler,
    output_trait::OutputTrait,
    seed_gen::consumers::base_consumer::Consumer,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OutputSeed;

impl<O> Consumer<O> for OutputSeed
where
    O: HasCallbackHandler,
{
    fn take(&self, seed_iter: &mut dyn Iterator<Item = f32>, output: &mut O) {
        output.output(OutputSeedIndexer::Seed(seed_iter.next().unwrap()));
    }
}
