use serde::{Deserialize, Serialize};

use crate::time::Time;


pub trait Split {

    fn get_name(&self) -> &str;
    fn get_time(&self) -> Time;

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedSplit {

    time: Time,
    name: String,

}

impl NamedSplit {

    pub fn new(time: Time, name: String) -> Self {
        Self {
            time,
            name
        }
    }

}

impl Split for NamedSplit {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_time(&self) -> Time {
        self.time
    }
}
