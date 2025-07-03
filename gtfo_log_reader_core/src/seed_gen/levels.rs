use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

use serde::Deserialize;

use crate::{core::data::LevelDescriptor, load_seed_consumers::LoadSeedConsumers, seed_gen::consumers::ConsumerEnum};

#[derive(Deserialize, Debug)]
pub struct LevelDescriptors {
    levels: HashMap<String, VecDeque<ConsumerEnum>>,
}

impl Default for LevelDescriptors {
    fn default() -> Self {
        Self { 
            levels: LoadSeedConsumers::load_all().unwrap()
        }
    }
}

impl LevelDescriptors {
    pub fn get_level(&self, level: &LevelDescriptor) -> Option<VecDeque<ConsumerEnum>> {
        self.levels.get(&level.to_string()).cloned()
    }
}
