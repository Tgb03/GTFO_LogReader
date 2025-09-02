use glr_core::seed_indexer_result::OutputSeedIndexer;
use serde::{Deserialize, Serialize};

use crate::{dll_exports::callback_handler::HasCallbackHandler, seed_gen::zone_info::{generated_data::{grab_spawn_id, AllocType}, objective::{StagedObjective, Task}, unlock_method::ZoneLocationSpawn}};
use crate::output_trait::OutputTrait;


#[derive(Debug, Serialize, Deserialize)]
pub struct CollectableAlloc {

    item_count: usize,
    max_per_zone: usize,
    skip_per_item: usize,
    objectives_in_zone: Vec<Vec<ZoneLocationSpawn>>,

    name: String,

}


impl<O: HasCallbackHandler> StagedObjective<O> for CollectableAlloc {
    fn get_task(&self, seed_iter: &mut dyn Iterator<Item = f32>, _: &mut O) -> Task<O> {
        let mut copied_zones: Vec<Vec<(ZoneLocationSpawn, usize)>> = self.objectives_in_zone.iter()
            .map(|v| 
                v.iter()
                    .map(|v| (v.clone(), self.max_per_zone))
                    .collect()
            )
            .collect();

        let mut p = Vec::new(); 
        for it in 0..self.item_count {
            let id = it % copied_zones.len();
            let options = &mut copied_zones[id];

            let zone_selected = (seed_iter.next().unwrap() * options.len() as f32) as usize;
            let selected = &mut options[zone_selected];
            
            selected.1 -= 1;
            p.push(selected.0.clone());

            options.retain(|v| v.1 > 0);
        }

        let cloned_name = self.name.clone();
        let skip_per_item = self.skip_per_item;
        
        Box::new(move |generated_zones, seed_iter, out| {
            for alloc in &p {
                if let Some(id) = grab_spawn_id(
                    generated_zones, 
                    alloc, 
                    AllocType::Container, 
                    seed_iter.next().unwrap(),
                ) {
                    out.output(OutputSeedIndexer::Key(cloned_name.clone(), alloc.zone_id.zone_id, id as i32));
                }

                if skip_per_item > 0 {
                    let _ = seed_iter.nth(skip_per_item - 1);
                }
            }
        })
    }
}

