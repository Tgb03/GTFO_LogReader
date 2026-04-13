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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum LockState {
    #[default]
    Unlocked,
    HackLock,
    BreakLock,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OutputSeedIndexer {
    Seed(f32),
    Key(String, u8, i32, i32),               // dimension, zone, id
    ResourcePack(ResourceType, u8, i32, i32, u8), // dimension, zone, id of box, pack size
    GenerationOverflow(usize),                // how many times the build seed went over in the level
    GenerationOverflowHash([u8; 32]),         // the hash for the generation overflow
    LockStateChange(u8, i32, i32, LockState), // dimension, zone, id
    LastContainerStateChange(LockState),      // dimension, zone, id
    GenerationEnd,
    GenerationStart(String),
    ZoneGenEnded(u32),

    ProcessFailed,
}
