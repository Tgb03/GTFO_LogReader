use glr_core::seed_indexer_result::OutputSeedIndexer;
use serde::{Deserialize, Serialize};

use crate::{output_trait::OutputTrait, seed_gen::consumers::base_consumer::Consumer};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct IgnoreConsumer {
    count: usize,
}

impl IgnoreConsumer {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}

impl<O> Consumer<O> for IgnoreConsumer
where
    O: OutputTrait<OutputSeedIndexer>,
{
    fn take(&self, seed_iter: &mut dyn Iterator<Item = f32>, _: &O) {
        if self.count > 0 {
            let _ = seed_iter.nth(self.count - 1);
        }
    }
}
