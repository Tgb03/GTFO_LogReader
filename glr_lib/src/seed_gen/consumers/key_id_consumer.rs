use glr_core::seed_indexer_result::OutputSeedIndexer;
use serde::{Deserialize, Serialize};

use crate::{
    output_trait::OutputTrait,
    seed_gen::consumers::base_consumer::Consumer,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyIDConsumer {
    #[serde(default)]
    name: String,
    #[serde(default)]
    zone: i32,

    #[serde(default)]
    start_weight: i32,
    #[serde(default)]
    middle_weight: i32,
    #[serde(default)]
    end_weight: i32,

    spawns_per_room: Vec<i32>,
}

impl<O> Consumer<O> for KeyIDConsumer
where
    O: OutputTrait<OutputSeedIndexer>,
{
    fn take(&self, seed_iter: &mut dyn Iterator<Item = f32>, output: &O) {
        let out = OutputSeedIndexer::Key(
            self.name.clone(),
            self.zone,
            self.get_id(seed_iter.next().unwrap()) as i32,
        );

        output.output(out);
    }
}

impl KeyIDConsumer {
    fn get_room(&self, seed: f32, values_per_room: &Vec<f32>) -> usize {
        for (i, count) in values_per_room.iter().enumerate() {
            if seed <= *count {
                return i;
            }
        }

        return values_per_room.len();
    }

    pub fn get_id(&self, seed: f32) -> usize {
        let values_per_room = self.calculate_values_per_room();

        let room = self.get_room(seed, &values_per_room);
        let spawn_count = self.spawns_per_room[room];
        let previous_room = match room > 0 {
            true => values_per_room[room - 1],
            false => 0f32,
        };
        let size = values_per_room[room] - previous_room;
        let left = seed - previous_room;

        let percent = left / size;
        let mut previous_room_total = 0;
        for i in 0..room {
            previous_room_total += self.spawns_per_room[i];
        }

        (percent * spawn_count as f32) as usize + previous_room_total as usize
    }

    fn calculate_weights(&self) -> Vec<f32> {
        let room_count = self.spawns_per_room.len();
        let mut room_weights = Vec::with_capacity(room_count);

        for i in 0..room_count {
            let weight_multis = Self::calculate_multipliers(i, room_count);

            room_weights.push(0f32);

            room_weights[i] = weight_multis[0] * self.start_weight as f32
                + weight_multis[1] * self.middle_weight as f32
                + weight_multis[2] * self.end_weight as f32
                + 1f32;
            room_weights[i] *= self.spawns_per_room[i] as f32;
        }

        room_weights
    }

    fn calculate_multipliers(area_id: usize, size: usize) -> [f32; 3] {
        if area_id * 2 == size - 1 {
            return [0f32, 1f32, 0f32];
        }

        if area_id < size / 2 {
            let mut weights = [0f32; 3];
            let a = f32::floor((size / 2) as f32);

            weights[0] = (a - area_id as f32) / a;
            weights[1] = 1f32 - weights[0];
            weights[2] = 0f32;

            return weights;
        }

        if area_id >= size / 2 {
            let mut weights = Self::calculate_multipliers(size - area_id - 1, size);
            weights.swap(0, 2);
            return weights;
        }

        [0f32, 0f32, 0f32]
    }

    fn calculate_values_per_room(&self) -> Vec<f32> {
        let weights = self.calculate_weights();
        let total_sum: f32 = weights.iter().sum();
        let mut values_per_id = vec![0f32; weights.len()];

        for i in 0..weights.len() {
            values_per_id[i] = weights[i] / total_sum;
            if i > 0 {
                values_per_id[i] += values_per_id[i - 1];
            }
        }

        values_per_id
    }
}
