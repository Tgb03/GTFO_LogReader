use std::collections::BTreeMap;
#[cfg(debug_assertions)]
use std::{fs::File, io::Read};

use crate::seed_gen::zone_info::level_data::LevelData;

pub struct LoadSeedConsumers;

impl LoadSeedConsumers {
    #[cfg(not(debug_assertions))]
    pub fn load_all() -> Option<BTreeMap<String, LevelData>> {
        let file_text = include_bytes!("..\\..\\interop\\level_descriptors.bin");

        match bincode::deserialize(file_text) {
            Ok(k) => {
                return Some(k);
            }
            Err(e) => {
                println!("{:?}", e);
                return None;
            }
        }
    }

    #[cfg(debug_assertions)]
    pub fn load_all() -> Option<BTreeMap<String, LevelData>> {
        let current_dir = std::env::current_dir()
            .ok()?
            .join("resources")
            .join("level_descriptors.json");
        println!("Path: {current_dir:?}");
        let mut file = File::open(current_dir).ok()?;
        let mut file_text = Vec::default();
        let _ = file.read_to_end(&mut file_text).ok()?;

        match serde_json::from_slice(&file_text) {
            Ok(k) => {
                return Some(k);
            }
            Err(e) => {
                println!("{:?}", e);
                return None;
            }
        }
    }
}
