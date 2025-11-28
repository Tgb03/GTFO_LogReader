use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CollectableMapper {
    map: HashMap<String, HashMap<u64, HashMap<u64, u64>>>,
}

impl CollectableMapper {
    pub fn load_from_file() -> Option<Self> {
        let baked = include_bytes!("..\\..\\..\\interop\\collectable_maps.bin");

        bincode::deserialize(baked).ok()
    }

    pub fn get_id(&self, level_name: &str, zone: u64, seed: u64) -> Option<u64> {
        //println!("Called: {} in {} at {}", level_name, zone, seed);

        self.map.get(level_name)?.get(&zone)?.get(&seed).cloned()
    }
}
