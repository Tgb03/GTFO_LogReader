use serde::{Deserialize, Serialize};

use crate::{
    dll_exports::callback_handler::HasCallbackHandler,
    seed_gen::consumers::{base_consumer::Consumer, key_id_consumer::KeyIDConsumer},
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ObjectiveConsumer {
    item_count: usize,
    max_per_zone: usize,
    objectives_in_zones: Vec<Vec<KeyIDConsumer>>,
}

impl<O> Consumer<O> for ObjectiveConsumer
where
    O: HasCallbackHandler,
{
    fn take(&self, seed_iter: &mut dyn Iterator<Item = f32>, output: &mut O) {
        for it in self.objectives_in_zones.iter() {
            let it = &it[(seed_iter.next().unwrap() * it.len() as f32) as usize];

            it.take(seed_iter, output);
        }
    }
}
