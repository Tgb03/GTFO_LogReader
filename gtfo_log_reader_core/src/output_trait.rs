use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::seed_gen::consumers::resource_generation::ResourceType;

pub trait OutputTrait<D> {
    fn output(&self, data: D);
}

#[derive(Default)]
pub struct PrintOutput;

impl<T: Debug> OutputTrait<T> for PrintOutput {
    fn output(&self, data: T) {
        println!("{:?}", data);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OutputSeedIndexer {
    Seed(f32),
    Key(String, i32, i32),           // zone, id
    ResourcePack(ResourceType, i32), // count
    ConsumableFound(i32, bool),      // id of box, found or not
    GenerationEnd,
    GenerationStart,
}
