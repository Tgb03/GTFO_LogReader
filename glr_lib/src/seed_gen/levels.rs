use std::{collections::BTreeMap, fmt::Debug};

use glr_core::data::LevelDescriptor;

use crate::{load_seed_consumers::SEED_LEVEL_DATAS, seed_gen::zone_info::level_data::LevelData};

#[derive(Debug)]
pub struct LevelDescriptors {
    levels: &'static Option<BTreeMap<String, LevelData>>,
}

impl Default for LevelDescriptors {
    fn default() -> Self {
        Self {
            levels: &SEED_LEVEL_DATAS,
        }
    }
}

impl LevelDescriptors {
    pub fn get_level(&self, level: &LevelDescriptor) -> Option<&LevelData> {
        self.levels.as_ref()?
            .get(&level.to_string())
    }
}
