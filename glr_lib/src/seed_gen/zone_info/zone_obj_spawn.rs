use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneObjectSpawn {

    pub name: String,

    pub start_weight: i32,
    pub middle_weight: i32,
    pub end_weight: i32,

}

impl Into<[i32; 3]> for &ZoneObjectSpawn {
    fn into(self) -> [i32; 3] {
        [
            self.start_weight,
            self.middle_weight,
            self.end_weight,
        ]
    }
}
