use glr_core::seed_indexer_result::OutputSeedIndexer;
use serde::{Deserialize, Serialize};

use crate::{
    output_trait::OutputTrait, seed_gen::consumers::{base_consumer::Consumer, key_id_consumer::KeyIDConsumer}
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ObjectiveConsumer {
    item_count: usize,
    max_per_zone: usize,
    objectives_in_zones: Vec<Vec<KeyIDConsumer>>,
}

impl<O> Consumer<O> for ObjectiveConsumer
where
    O: OutputTrait<OutputSeedIndexer>,
{
    fn take(&self, seed_iter: &mut dyn Iterator<Item = f32>, output: &O) {
        for it in self.objectives_in_zones.iter() {
            let it = &it[(seed_iter.next().unwrap() * it.len() as f32) as usize];

            it.take(seed_iter, output);
        }
    }
}
