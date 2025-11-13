use std::{collections::{HashMap, VecDeque}};

#[cfg(debug_assertions)]
use std::{fs::File, io::Read};

use crate::seed_gen::consumers::ConsumerEnum;


pub struct LoadSeedConsumers;

impl LoadSeedConsumers {

    #[cfg(not(debug_assertions))]
    pub fn load_all() -> Option<HashMap<String, VecDeque<ConsumerEnum>>> {
        let file_text = include_bytes!("..\\..\\interop\\level_descriptors.bin");

        match bincode::deserialize(file_text) {
            Ok(k) => { return Some(k); },
            Err(e) => { println!("{:?}", e); return None; },
        }
    }
    
    #[cfg(debug_assertions)]
    pub fn load_all() -> Option<HashMap<String, VecDeque<ConsumerEnum>>> {
        let current_dir = std::env::current_dir().ok()?
            .join("resources")
            .join("level_descriptors.json");
        let mut file = File::open(current_dir).ok()?;
        let mut file_text = Vec::default();
        let _ = file.read_to_end(&mut file_text).ok()?;
        
        match bincode::deserialize(&file_text) {
            Ok(k) => { return Some(k); },
            Err(e) => { println!("{:?}", e); return None; },
        }
    }

}
