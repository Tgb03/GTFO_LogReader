use glr_core::seed_indexer_result::OutputSeedIndexer;
use serde::{Deserialize, Serialize};

use crate::output_trait::OutputTrait;
use crate::seed_gen::marker_set::MarkerSetHash;
use crate::{
    dll_exports::callback_handler::HasCallbackHandler,
    seed_gen::zone_info::{
        generated_data::{AllocType, GeneratedZone, grab_spawn_id},
        unlock_method::ZoneLocationSpawn,
        zone_identifier::ZoneIdentifier,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SpawnObject {
    pub name: String,
    pub zone_id: ZoneIdentifier,

    pub start_weight: i32,
    pub middle_weight: i32,
    pub end_weight: i32,

    pub alloc_type: AllocType,
    #[serde(default)] pub skip_before_alloc: usize,
}

impl SpawnObject {
    pub fn take<O: HasCallbackHandler>(
        self,
        generated_data: &mut Vec<GeneratedZone>,
        seed_iter: &mut dyn Iterator<Item = f32>,
        build_seeds: &mut impl Iterator<Item = f32>,
        overflow_counter: &mut MarkerSetHash,
        output: &mut O,
    ) -> Option<()> {
        let is_container = self.alloc_type == AllocType::Container;
        let location = ZoneLocationSpawn {
            zone_id: self.zone_id,
            start_weight: self.start_weight,
            middle_weight: self.middle_weight,
            end_weight: self.end_weight,
        };
        
        if self.skip_before_alloc > 0 {
            let _ = seed_iter.nth(self.skip_before_alloc - 1);
        }
        
        let id = grab_spawn_id(
            generated_data,
            &location,
            self.alloc_type,
            seed_iter,
            build_seeds,
            overflow_counter,
            Some(format!("SpawnObject alloc: {}", self.name).as_str()),
            false,
        )?;

        if is_container {
            let _ = seed_iter.next();
        }

        output.output(OutputSeedIndexer::Key(
            self.name,
            self.zone_id.zone_id,
            id as i32,
        ));

        Some(())
    }
}
