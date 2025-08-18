use serde::{Deserialize, Serialize};

use crate::seed_gen::zone_info::{generated_data::AllocType, zone_identifier::ZoneIdentifier};



#[derive(Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum UnlockMethodType {

    None,
    ColoredKey,
    BulkheadKey,
    Cell,

}

impl TryInto<AllocType> for &UnlockMethodType {
    type Error = ();
    
    fn try_into(self) -> Result<AllocType, Self::Error> {
        match self {
            UnlockMethodType::None => Err(()),
            UnlockMethodType::ColoredKey | UnlockMethodType::BulkheadKey => Ok(AllocType::Container),
            UnlockMethodType::Cell => Ok(AllocType::BigPickup),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZoneLocationSpawn {

    #[serde(flatten)]
    pub zone_id: ZoneIdentifier,

    pub start_weight: i32,
    pub middle_weight: i32,
    pub end_weight: i32,

}


#[derive(Debug, Serialize, Deserialize)]
pub struct UnlockMethod {

    pub unlock_type: UnlockMethodType,
    pub zones: Vec<ZoneLocationSpawn>,

}

impl UnlockMethod {

    pub fn grab_zone(&self, seed: f32) -> &ZoneLocationSpawn {
        &self.zones[(seed * self.zones.len() as f32) as usize]
    }

}
