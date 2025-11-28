use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::seed_gen::zone_info::{
    unlock_method::ZoneLocationSpawn,
    zone_data::{RoomSize, ZoneData},
    zone_identifier::ZoneIdentifier,
};

#[derive(Debug)]
pub struct GeneratedZone {
    zone_id: ZoneIdentifier,

    alloc_containers: Vec<Vec<(u8, u8)>>,
    alloc_small_pickups: Vec<Vec<(u8, u8)>>,
    alloc_big_pickups: Vec<Vec<(u8, u8)>>,

    alloc_terminals: Vec<Vec<(u8, u8)>>,
    alloc_other: Vec<Vec<(u8, u8)>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AllocType {
    Container = 0,
    SmallPickup = 1,
    BigPickup = 2,

    Terminal = 3,
    Other = 4,
}

impl GeneratedZone {
    pub fn new(build_seeds: &mut impl Iterator<Item = f32>, zone_data: &ZoneData) -> Self {
        Self {
            zone_id: zone_data.zone_id.clone(),
            alloc_containers: Self::create_container_alloc(
                build_seeds,
                &zone_data.rooms,
                zone_data.allow_containers_alloc,
            ),
            alloc_small_pickups: Self::create_small_pickups_alloc(
                build_seeds,
                &zone_data.rooms,
                zone_data.allow_small_pickups,
            ),
            alloc_big_pickups: Self::create_big_pickups_alloc(
                build_seeds,
                &zone_data.rooms,
                zone_data.allow_big_pickups,
            ),
            alloc_terminals: Self::initial_allocations_from_vec(&zone_data.terminals, 1),
            alloc_other: Self::initial_allocations_from_vec(&zone_data.alloc_other, 1),
        }
    }

    fn create_container_alloc(
        build_seeds: &mut impl Iterator<Item = f32>,
        rooms: &Vec<RoomSize>,
        generate: bool,
    ) -> Vec<Vec<(u8, u8)>> {
        if !generate {
            return Vec::new();
        }

        let mut result = Vec::with_capacity(rooms.len());
        let mut start = 0;

        for room in rooms {
            let room_len = room.into_containers();
            let mut append_res = Vec::with_capacity(room_len as usize * 2);

            for id in 0..room_len {
                let big_box = (build_seeds.nth(2).unwrap() * 2f32) as u8 + 2;
                // println!("Container generated with {} alloc", big_box);
                append_res.push((id + start, big_box));
            }

            start += room_len;
            result.push(append_res);
        }

        result
    }

    fn create_small_pickups_alloc(
        build_seeds: &mut impl Iterator<Item = f32>,
        rooms: &Vec<RoomSize>,
        generate: bool,
    ) -> Vec<Vec<(u8, u8)>> {
        if !generate {
            return Vec::new();
        }

        let mut result = Vec::with_capacity(rooms.len());
        let mut start = 0;

        for room in rooms {
            let room_len = room.into_small_pickups();
            let mut append_res = Vec::with_capacity(room_len as usize);

            for id in 0..room_len {
                let seed = build_seeds.next().unwrap();
                // println!("Small pickup seed: {}", (seed * 2147483647f32) as usize);
                append_res.push((id + start, 1));
            }

            start += room_len;
            result.push(append_res);
        }

        result
    }

    fn create_big_pickups_alloc(
        build_seeds: &mut impl Iterator<Item = f32>,
        rooms: &Vec<RoomSize>,
        generate: bool,
    ) -> Vec<Vec<(u8, u8)>> {
        if !generate {
            return Vec::new();
        }

        let mut result = Vec::with_capacity(rooms.len());
        let mut start = 0;

        for room in rooms {
            let room_len = room.into_big_pickups();
            let mut append_res = Vec::with_capacity(room_len as usize);

            for id in 0..room_len {
                let _ = build_seeds.next();
                append_res.push((id + start, 1));
            }

            start += room_len;
            result.push(append_res);
        }

        result
    }

    fn initial_allocations_from_vec(rooms: &Vec<u8>, per_id: u8) -> Vec<Vec<(u8, u8)>> {
        let mut result = Vec::with_capacity(rooms.len());
        let mut start = 0;

        for room in rooms {
            let (room_len, alloc_per_space) = (*room, per_id as usize);

            let mut append_res = Vec::with_capacity(room_len as usize * alloc_per_space);

            for id in 0..room_len {
                append_res.push((id + start, alloc_per_space as u8));
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
        seed_iter: &mut dyn Iterator<Item = f32>,
        build_seeds: &mut impl Iterator<Item = f32>,
        _debug_str: Option<&str>,
        check_alloc: bool,
    ) -> isize {
        // disabled for now as ALL of this is just testing
        // if *alloc_type == AllocType::BigPickup && check_alloc {
        //     return -1
        // }

        let spawns_per_room: Vec<usize> = match alloc_type {
            AllocType::Container => &mut self.alloc_containers,
            AllocType::SmallPickup => &mut self.alloc_small_pickups,
            AllocType::BigPickup => &mut self.alloc_big_pickups,
            AllocType::Other => &mut self.alloc_other,
            AllocType::Terminal => &mut self.alloc_terminals,
        }
        .iter()
        .map(|v| {
            v.len()
        })
        .collect();

        let total = spawns_per_room.iter().fold(0, |r, v| r + v);
        let seed = seed_iter.next().unwrap();
        
        if total == 0 {
            // println!("     extra allocation done");
            let _ = match alloc_type {
                AllocType::Container => build_seeds.nth(3),
                AllocType::SmallPickup => build_seeds.nth(1),
                AllocType::BigPickup => build_seeds.next(),
                AllocType::Terminal => None,
                AllocType::Other => None,
            };
            
            return -1;
        }

        let values_per_room = Self::calculate_values_per_room(&spawns_per_room, weights);

        // match _debug_str {
        //     Some(s) => println!("s: {} from {}", seed, s),
        //     None => println!("s: {}", seed),
        // }
        let mut room = Self::get_room(seed, &values_per_room);
        if room >= spawns_per_room.len() {
            room -= 1;
        }
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
            AllocType::Other => &mut self.alloc_other,
            AllocType::Terminal => &mut self.alloc_terminals,
        };

        let id = vec
            .get(room)
            .map(|v| v.get(in_room_id))
            .flatten()
            .map(|(id, _)| id)
            .cloned();

        vec.get_mut(room)
            .map(|v| {
                v.get_mut(in_room_id as usize)
                    .map(|(_, a)| {
                        *a -= 1;
                    });
                
                v.retain(|(_, v)| *v > 0);
        });

        id.unwrap_or_default() as isize
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
    seed_iter: &mut dyn Iterator<Item = f32>,
    build_seeds: &mut impl Iterator<Item = f32>,
    overflow_counter: &mut usize,
    debug_str: Option<&str>,
    check_alloc: bool,
) -> Option<isize> {
    if let Some(zone) = zones
        .iter_mut()
        .filter(|v| v.zone_id == spawn.zone_id)
        .nth(0)
    {
        let id = zone.spawn_id(
            &[spawn.start_weight, spawn.middle_weight, spawn.end_weight],
            &alloc_type,
            seed_iter,
            build_seeds,
            debug_str,
            check_alloc,
        );
        if id == -1 { *overflow_counter += 1; }
        
        Some(id)
    } else {
        let _ = seed_iter.next();

        None
    }
}
