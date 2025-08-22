

use glr_core::seed_indexer_result::ResourceType;
use serde::{Deserialize, Serialize};

use crate::{
    dll_exports::callback_handler::HasCallbackHandler, seed_gen::consumers::{
        base_consumer::Consumer, ignore_consumer::IgnoreConsumer, resource_generation::ResourceGeneration
    }
};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneConsumer {

    zone_index: u8,
    shown_number: u32,

    medi: f32,
    disi: f32,
    ammo: f32,
    tool: f32,

    artifact_count: u32,
    consumable_in_container: u32,
    consumable_in_worldspawn: u32,

}


impl<O> Consumer<O> for ZoneConsumer
where
    O: HasCallbackHandler, {
    
    fn take(&self, seed_iter: &mut dyn Iterator<Item = f32>, output: &mut O) {
        ResourceGeneration::new(self.medi, ResourceType::Healthpack, None)
            .take(seed_iter, output);
        ResourceGeneration::new(self.disi, ResourceType::DisinfectPack, None)
            .take(seed_iter, output);
        ResourceGeneration::new(self.ammo, ResourceType::Ammopack, None)
            .take(seed_iter, output);
        ResourceGeneration::new(self.tool, ResourceType::ToolRefillpack, None)
            .take(seed_iter, output);
        IgnoreConsumer::new(
            (self.artifact_count + self.consumable_in_container * 2 + self.consumable_in_worldspawn) as usize
        )
            .take(seed_iter, output);
    }
}