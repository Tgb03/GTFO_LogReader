use serde::{Deserialize, Serialize};

use crate::seed_gen::zone_info::generated_data::AllocType;



#[derive(Debug, Serialize, Deserialize)]
pub struct SpawnObject {

    name: String,
    
    start_weight: i32,
    middle_weight: i32,
    end_weight: i32,

    alloc_type: AllocType,

}


