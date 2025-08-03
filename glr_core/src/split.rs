use serde::{Deserialize, Serialize};

use crate::time::Time;


pub trait Split {

    fn get_name(&self) -> &str;
    fn get_time(&self) -> Time;

}


impl<S: Split> Split for Vec<S> {
    fn get_name(&self) ->  &str {
        ""
    }

    fn get_time(&self) -> Time {
        self.iter()
            .map(|v| v.get_time())
            .fold(Time::new(), |a, b| a + b)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
