

use serde::{Deserialize, Serialize};

use crate::{
    dll_exports::callback_handler::HasCallbackHandler,
    seed_gen::consumers::{base_consumer::Consumer, key_id_consumer::KeyIDConsumer},
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum KeyType {
    #[default]
    ColoredKey,
    BulkheadKey,
    Other
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct KeyConsumer {
    key_type: KeyType,
    zones: Vec<KeyIDConsumer>,
}

impl KeyConsumer {
    pub fn get_first_id(&self) -> usize {
        match self.key_type {
            KeyType::ColoredKey => 2,
            KeyType::BulkheadKey => 1,
            KeyType::Other => 0,
        }
    }
}

impl<O> Consumer<O> for KeyConsumer
where
    O: HasCallbackHandler,
{
    fn take(&self, seed_iter: &mut dyn Iterator<Item = f32>, output: &mut O) {
        let zone = (seed_iter.nth(self.get_first_id()).unwrap() * self.zones.len() as f32) as usize;
        self.zones[zone].take(seed_iter, output);
    }
}
