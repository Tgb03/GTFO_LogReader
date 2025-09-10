use glr_core::seed_indexer_result::OutputSeedIndexer;
use serde::{Deserialize, Serialize};

use crate::{dll_exports::callback_handler::HasCallbackHandler, seed_gen::zone_info::{generated_data::{grab_spawn_id, AllocType, GeneratedZone}, unlock_method::ZoneLocationSpawn, zone_identifier::ZoneIdentifier}};
use crate::output_trait::OutputTrait;


#[derive(Debug, Serialize, Deserialize)]
pub struct SpawnObject {

    pub name: String,
    pub zone_id: ZoneIdentifier,
    
    pub start_weight: i32,
    pub middle_weight: i32,
    pub end_weight: i32,

    pub alloc_type: AllocType,

}


impl SpawnObject {

    pub fn take<O: HasCallbackHandler>(
        self,
        generated_data: &mut Vec<GeneratedZone>,
        seed_iter: &mut dyn Iterator<Item = f32>,
        output: &mut O,
    ) -> Option<()> {
        let is_container = self.alloc_type == AllocType::Container;
        let seed = seed_iter.next()?;
        let id = grab_spawn_id(
            generated_data, 
            &ZoneLocationSpawn {
                zone_id: self.zone_id,
                start_weight: self.start_weight,
                middle_weight: self.middle_weight,
                end_weight: self.end_weight,
            }, 
            self.alloc_type, 
            seed
        )?;

        if is_container { let _ = seed_iter.next(); }

        output.output(OutputSeedIndexer::Key(self.name, self.zone_id.zone_id, id as i32));

        Some(())
    }

}


