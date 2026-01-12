use std::collections::HashSet;

use crate::output_trait::OutputTrait;
use glr_core::seed_indexer_result::OutputSeedIndexer;
use serde::{Deserialize, Serialize};

use crate::seed_gen::consumers::base_consumer::Consumer;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsumableConsumer {
    tracked_containers: Vec<i32>,
    total_container_count: i32,
    consumable_count: i32,
}

impl<O> Consumer<O> for ConsumableConsumer
where
    O: OutputTrait<OutputSeedIndexer>,
{
    fn take(&self, seed_iter: &mut dyn Iterator<Item = f32>, output: &O) {
        let mut found_counters = HashSet::<i32>::new();

        for _ in 0..self.consumable_count {
            let id = (seed_iter.next().unwrap() * self.total_container_count as f32) as i32;
            let _ = seed_iter.next();

            found_counters.insert(id);
        }

        for id in &self.tracked_containers {
            output.output(OutputSeedIndexer::ConsumableFound(
                *id,
                found_counters.contains(id),
            ));
        }
    }
}
