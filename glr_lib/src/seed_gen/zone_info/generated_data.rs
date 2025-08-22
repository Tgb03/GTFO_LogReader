
use serde::{Deserialize, Serialize};

use crate::seed_gen::zone_info::{unlock_method::ZoneLocationSpawn, zone_data::ZoneData, zone_identifier::ZoneIdentifier};


#[derive(Debug)]
pub struct GeneratedZone {

    zone_id: ZoneIdentifier,
    
    alloc_containers: Vec<Vec<(u8, u8)>>,
    alloc_small_pickups: Vec<Vec<(u8, u8)>>,
    alloc_big_pickups: Vec<Vec<(u8, u8)>>,

}


#[derive(Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum AllocType {

    Container = 0,
    SmallPickup = 1,
    BigPickup = 2,

}


impl From<&ZoneData> for GeneratedZone {
    fn from(value: &ZoneData) -> Self {
        Self {
            zone_id: value.zone_id.clone(),
            alloc_containers: value.rooms.iter()
                .map(|v| 
                    (0..v.into_containers())
                )
                .scan(0, |state: &mut u8, v| {
                    let range_start = *state;

                    *state = range_start + v.end;
                    Some(range_start..range_start + v.end)
                })
                .map(|v| 
                    v.map(|v| (v, 3u8))
                )
                .map(|v| v.collect())
                .collect(),
            alloc_small_pickups: value.rooms.iter()
                .map(|v| 
                    (0..v.into_small_pickups())
                )
                .scan(0, |state: &mut u8, v| {
                    let range_start = *state;

                    *state = range_start + v.end;
                    Some(range_start..range_start + v.end)
                })
                .map(|v| 
                    v.map(|v| (v, 1u8))
                )
                .map(|v| v.collect())
                .collect(),
            alloc_big_pickups: value.rooms.iter()
                .map(|v| 
                    (0..v.into_big_pickups())
                )
                .scan(0, |state: &mut u8, v| {
                    let range_start = *state;

                    *state = range_start + v.end;
                    Some(range_start..range_start + v.end)
                })
                .map(|v| 
                    v.map(|v| (v, 1u8))
                )
                .map(|v| v.collect())
                .collect(),
        }
    }
}

impl GeneratedZone {

    /// someone remake this entire thing
    /// if i continue writing in this i will kill myself
    pub fn spawn_id(
        &mut self, 
        weights: &[i32; 3], 
        alloc_type: &AllocType, 
        seed: f32, 
        advanced_checks: bool,
    ) -> usize {
        let mut spawns_per_room: Vec<usize> = match alloc_type {
            AllocType::Container => &mut self.alloc_containers,
            AllocType::SmallPickup => &mut self.alloc_small_pickups,
            AllocType::BigPickup => &mut self.alloc_big_pickups,
        }
            .iter()
            .map(|v| 
                v.len()
            )
            .collect();
        let values_per_room = Self::calculate_values_per_room(&spawns_per_room, weights);
        spawns_per_room = match alloc_type {
            AllocType::Container => &mut self.alloc_containers,
            AllocType::SmallPickup => &mut self.alloc_small_pickups,
            AllocType::BigPickup => &mut self.alloc_big_pickups,
        }
            .iter()
            .map(|v| 
                match advanced_checks {
                    true => v.iter()
                        .fold(0usize, |a, b| a + b.1 as usize),
                    false => v.len(),
                }
            )
            .collect();

        let room = Self::get_room(seed, &values_per_room);
        let spawn_count = spawns_per_room[room];
        let previous_room = match room > 0 {
            true => values_per_room[room - 1],
            false => 0f32,
        };
        let size = values_per_room[room] - previous_room;
        let left = seed - previous_room;

        let percent = left / size;
        let mut in_room_id = (percent * spawn_count as f32) as usize;

        let vec = match alloc_type {
            AllocType::Container => &mut self.alloc_containers,
            AllocType::SmallPickup => &mut self.alloc_small_pickups,
            AllocType::BigPickup => &mut self.alloc_big_pickups,
        };

        let id = match advanced_checks {
            false => vec.get_mut(room)
                .map(|v| v.get_mut(in_room_id))
                .flatten()
                .map(|(id, c)| {
                    *c -= 1;
                    
                    *id as usize
                }).unwrap(),
            true => vec.get_mut(room)
                .map(|v| {
                    for it in v {
                        if in_room_id < it.1 as usize {
                            it.1 -= 1;
                            return Some(it.0 as usize);
                        }

                        in_room_id -= it.1 as usize;
                    }

                    None
                })
                .flatten()
                .unwrap(),
        };

        vec.get_mut(room)
            .unwrap()
            .retain(|v| v.1 != 0);

        id as usize
    }

    fn get_room(seed: f32, values_per_room: &Vec<f32>) -> usize {

        for (i, count) in values_per_room.iter().enumerate() {
            if seed <= *count {
                return i;
            }
        }

        return values_per_room.len();
    }

    fn calculate_weights(spawns_per_room: &Vec<usize>, weights: &[i32; 3]) -> Vec<f32> {
        let room_count = spawns_per_room.len();
        let mut room_weights = Vec::with_capacity(room_count);

        for i in 0..room_count {
            let weight_multis = Self::calculate_multipliers(i, room_count);

            room_weights.push(0f32);

            room_weights[i] = weight_multis[0] * weights[0] as f32
                + weight_multis[1] * weights[1] as f32
                + weight_multis[2] * weights[2] as f32
                + 1f32;
            room_weights[i] *= spawns_per_room[i] as f32;
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

    fn calculate_values_per_room(spawns_per_room: &Vec<usize>, weights: &[i32; 3]) -> Vec<f32> {
        let weights = Self::calculate_weights(spawns_per_room, weights);
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


pub fn grab_spawn_id(
    zones: &mut Vec<GeneratedZone>, 
    spawn: &ZoneLocationSpawn, 
    alloc_type: AllocType, 
    seed: f32,
    advanced_checks: bool,
) -> Option<usize> {
    
    zones.iter_mut()
        .filter(|v| v.zone_id == spawn.zone_id)
        .nth(0)
        .map(|v| v.spawn_id(
            &[spawn.start_weight, spawn.middle_weight, spawn.end_weight], 
            &alloc_type, 
            seed,
            advanced_checks
        ))
}

