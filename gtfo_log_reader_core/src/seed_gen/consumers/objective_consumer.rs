

use serde::{Deserialize, Serialize};

use crate::{dll_exports::callback_handler::HasCallbackHandler, seed_gen::consumers::{base_consumer::Consumer, key_id_consumer::KeyIDConsumer}};



#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ObjectiveConsumer {
    item_count: usize,
    max_per_zone: usize,
    skip_per_item: usize,
    objectives_in_zones: Vec<Vec<KeyIDConsumer>>,

    #[serde(skip_serializing, default)]
    picked_zones: Vec<KeyIDConsumer>,
    #[serde(skip_serializing, default)]
    objective_buffer: Vec<Vec<(KeyIDConsumer, usize)>>,
    #[serde(skip_serializing, default)]
    is_calculated: bool,
    #[serde(skip_serializing, default)]
    placement_data_index: usize,
}

impl ObjectiveConsumer {

    fn calculate_initial(&mut self) {
        if self.is_calculated { return; }

        self.skip_per_item += 1;
        self.is_calculated = true;

        self.objectives_in_zones.iter()
            .for_each(|v| {
                self.objective_buffer.push(
                    v.iter()
                        .map(|v| (v.clone(), self.max_per_zone))
                        .collect()
                );
            });
    }

    fn calc_zone(&mut self, seed: f32) -> Option<()> {
        let objective = self.objective_buffer.get_mut(self.placement_data_index)?;
        let picked = (objective.len() as f32 * seed) as usize;

        let (key_consumer, count) = objective.get_mut(picked)?;
        self.picked_zones.push(key_consumer.clone());
        
        *count -= 1;
        self.item_count -= 1;

        objective.retain(|v| v.1 != 0);

        self.placement_data_index += 1;
        if self.placement_data_index == self.objective_buffer.len() {
            self.placement_data_index = 0;
        }

        if self.item_count == 0 {
            self.picked_zones.reverse();
            self.placement_data_index = 0;
        }

        Some(())
        /* 
        if let Some(mut objectives_in_zone) = self.objectives_in_zones.pop() {
            let picked = (objectives_in_zone.len() as f32 * seed) as usize;
            self.picked_zones.push_back(objectives_in_zone.swap_remove(picked));
        }
        */
    }

}

impl<O> Consumer<O> for ObjectiveConsumer
where
    O: HasCallbackHandler, {
        
    fn take(&mut self, seed: f32, output: &mut O) -> bool {
        self.calculate_initial();

        if self.item_count > 0 {
            self.calc_zone(seed);
            return false;
        }

        if self.picked_zones.len() > 0 && self.placement_data_index % self.skip_per_item == 0 {
            println!("seed: {seed}");
            self.picked_zones.pop()
                .map(|mut z| z.take(seed, output));
        }

        self.placement_data_index += 1;

        self.item_count == 0 && self.picked_zones.is_empty()
    }

}

