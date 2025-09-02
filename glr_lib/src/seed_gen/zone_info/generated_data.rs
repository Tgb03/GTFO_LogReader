
use serde::{Deserialize, Serialize};

use crate::seed_gen::zone_info::{unlock_method::ZoneLocationSpawn, zone_data::{RoomSize, ZoneData}, zone_identifier::ZoneIdentifier};


#[derive(Debug)]
pub struct GeneratedZone {

    zone_id: ZoneIdentifier,
    
    alloc_containers: Vec<Vec<u8>>,
    alloc_small_pickups: Vec<Vec<u8>>,
    alloc_big_pickups: Vec<Vec<u8>>,

}


#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
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
            alloc_containers: Self::initial_allocations(AllocType::Container, &value.rooms),
            alloc_small_pickups: Self::initial_allocations(AllocType::SmallPickup, &value.rooms),
            alloc_big_pickups: Self::initial_allocations(AllocType::BigPickup, &value.rooms),
        }
    }
}

impl GeneratedZone {

    fn initial_allocations(
        alloc_type: AllocType,
        rooms: &Vec<RoomSize>,
    ) -> Vec<Vec<u8>> {
        let mut result = Vec::with_capacity(rooms.len());
        let mut start = 0;

        for room in rooms {
            let (room_len, alloc_per_space) = match alloc_type {
                AllocType::Container => (room.into_containers(), 3),
                AllocType::SmallPickup => (room.into_small_pickups(), 1),
                AllocType::BigPickup => (room.into_big_pickups(), 1),
            };

            let mut append_res = Vec::with_capacity(room_len as usize * alloc_per_space);

            for id in 0..room_len {
                for _ in 0..alloc_per_space {
                    append_res.push(id + start);
                }
            }

            start += room_len;
            result.push(append_res);
        }

        result    
    }

    pub fn spawn_id(
        &mut self, 
        weights: &[i32; 3], 
        alloc_type: &AllocType, 
        seed: f32,
    ) -> usize {
        let spawns_per_room: Vec<usize> = match alloc_type {
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

        let room = Self::get_room(seed, &values_per_room);
        let spawn_count = spawns_per_room[room];
        let previous_room = match room > 0 {
            true => values_per_room[room - 1],
            false => 0f32,
        };
        let size = values_per_room[room] - previous_room;
        let left = seed - previous_room;

        let percent = left / size;
        let in_room_id = (percent * spawn_count as f32) as usize;

        let vec = match alloc_type {
            AllocType::Container => &mut self.alloc_containers,
            AllocType::SmallPickup => &mut self.alloc_small_pickups,
            AllocType::BigPickup => &mut self.alloc_big_pickups,
        };

        let id = vec.get_mut(room)
            .map(|v| v.get_mut(in_room_id))
            .flatten()
            .map(|id| *id)
            .unwrap();

        // vec.get_mut(room)
        //     .map(|v| v.remove(in_room_id));

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
) -> Option<usize> {
    
    zones.iter_mut()
        .filter(|v| v.zone_id == spawn.zone_id)
        .nth(0)
        .map(|v| v.spawn_id(
            &[spawn.start_weight, spawn.middle_weight, spawn.end_weight], 
            &alloc_type, 
            seed,
        ))
}

