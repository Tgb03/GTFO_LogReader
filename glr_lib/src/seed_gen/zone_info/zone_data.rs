
use serde::{Deserialize, Serialize};

use crate::seed_gen::zone_info::{generated_data::AllocType, unlock_method::UnlockMethod, zone_identifier::ZoneIdentifier, zone_obj_spawn::ZoneObjectSpawn};


#[derive(Debug, Serialize, Deserialize)]
pub struct ZoneData {

    #[serde(flatten)]
    pub zone_id: ZoneIdentifier,
    pub unlocked_by: UnlockMethod,
    pub rooms: Vec<RoomSize>,

    #[serde(default)] pub terminals: Vec<u8>,
    #[serde(default)] pub alloc_other: Vec<u8>,
    
    pub medi: f32,
    #[serde(default)] pub medi_weights: [i32; 3],
    pub disi: f32,
    #[serde(default)] pub disi_weights: [i32; 3],
    pub ammo: f32,
    #[serde(default)] pub ammo_weights: [i32; 3],
    pub tool: f32,
    #[serde(default)] pub tool_weights: [i32; 3],
    
    pub consumables: Vec<ContainerOrWorldspawn>,
    pub artifacts: Vec<ContainerOrWorldspawn>,
    
    pub small_pickups: Vec<ZoneObjectSpawn>,
    pub big_pickups: Vec<ZoneObjectSpawn>,
    #[serde(default)] pub other_pickups: Vec<ZoneObjectSpawn>,

}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
pub enum ContainerOrWorldspawn {

    Container,
    Worldspawn,

}

impl Into<AllocType> for &ContainerOrWorldspawn {
    fn into(self) -> AllocType {
        match self {
            ContainerOrWorldspawn::Container => AllocType::Container,
            ContainerOrWorldspawn::Worldspawn => AllocType::SmallPickup,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum RoomSize {

    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Other(u8, u8, u8),

}

impl RoomSize {

    pub fn into_containers(&self) -> u8 {
        match self {
            RoomSize::Tiny => 1,
            RoomSize::Small => 4,
            RoomSize::Medium => 6,
            RoomSize::Large => 9,
            RoomSize::Huge => 14,
            RoomSize::Other(v, _, _) => *v,
        }
    }

    pub fn into_small_pickups(&self) -> u8 {
        match self {
            RoomSize::Tiny => 2,
            RoomSize::Small => 3,
            RoomSize::Medium => 6,
            RoomSize::Large => 8,
            RoomSize::Huge => 10,
            RoomSize::Other(_, v, _) => *v,
        }
    }

    pub fn into_big_pickups(&self) -> u8 {
        match self {
            RoomSize::Tiny => 1,
            RoomSize::Small => 1,
            RoomSize::Medium => 2,
            RoomSize::Large => 3,
            RoomSize::Huge => 5,
            RoomSize::Other(_, _, v) => *v,
        }
    }

}
