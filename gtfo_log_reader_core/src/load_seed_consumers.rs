use std::{collections::{HashMap, VecDeque}, fs::File, io::Read};

use crate::seed_gen::consumers::ConsumerEnum;


pub struct LoadSeedConsumers;

impl LoadSeedConsumers {

    /*
    pub fn load_all() -> Option<HashMap<String, VecDeque<ConsumerEnum>>> {
        let file_text = include_str!("..\\..\\resources\\level_descriptors.json");

        match serde_json::from_str(file_text) {
            Ok(k) => { return Some(k); },
            Err(e) => { println!("{:?}", e); return None; },
        }
    }
    */

    pub fn load_all() -> Option<HashMap<String, VecDeque<ConsumerEnum>>> {
        let current_dir = std::env::current_dir().ok()?
            .join("resources")
            .join("level_descriptors.json");
        println!("Path: {:?}", current_dir);
        let mut file = File::open(current_dir).ok()?;
        let mut file_text = String::default();
        let _ = file.read_to_string(&mut file_text).ok()?;
        
        match serde_json::from_str(&file_text) {
            Ok(k) => { return Some(k); },
            Err(e) => { println!("{:?}", e); return None; },
        }
    }

}
