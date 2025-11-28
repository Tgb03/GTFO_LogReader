use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum ResourceType {
    #[default]
    Healthpack,
    DisinfectPack,
    Ammopack,
    ToolRefillpack,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OutputSeedIndexer {
    Seed(f32),
    Key(String, i32, i32),                    // zone, id
    ResourcePack(ResourceType, i32, i32, u8), // zone, id of box, pack size
    ConsumableFound(i32, bool),               // id of box, found or not
    GenerationOverflow(usize),                // how many times the build seed went over in the level
    GenerationEnd,
    GenerationStart(String),
    ZoneGenEnded(u32),

    ProcessFailed,
}
