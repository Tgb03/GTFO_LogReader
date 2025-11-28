use std::{collections::BTreeMap, fmt::Debug};

use glr_core::data::LevelDescriptor;
use serde::Deserialize;

use crate::{load_seed_consumers::LoadSeedConsumers, seed_gen::zone_info::level_data::LevelData};

#[derive(Deserialize, Debug)]
pub struct LevelDescriptors {
    levels: BTreeMap<String, LevelData>,
}

impl Default for LevelDescriptors {
    fn default() -> Self {
        Self {
            levels: LoadSeedConsumers::load_all().unwrap(),
        }
    }
}

impl LevelDescriptors {
    pub fn get_level(&self, level: &LevelDescriptor) -> Option<&LevelData> {
        self.levels.get(&level.to_string())
    }
}
