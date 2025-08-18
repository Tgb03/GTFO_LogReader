use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct ZoneIdentifier {

    pub layer_id: u8,
    pub dimension_id: u8,
    pub zone_id: i32,

}
