use serde::{Deserialize, Serialize};

use crate::{core::{data::LevelDescriptor, time::Time}, run_gen::split::Split};


#[derive(Debug, Serialize, Deserialize)]
pub struct TimedRun<S>
where 
    S: Split {

    name: LevelDescriptor,
    total_time: Time,
    player_count: u8,

    used_checkpoint: bool,
    is_win: bool,

    did_secondary: bool,
    did_overload: bool,

    splits: Vec<S>,

}

impl<S: Split> Default for TimedRun<S> {
    fn default() -> Self {
        Self { 
            name: Default::default(),
            total_time: Default::default(), 
            player_count: Default::default(), 
            used_checkpoint: Default::default(), 
            is_win: Default::default(), 
            did_secondary: Default::default(), 
            did_overload: Default::default(),
            splits: Default::default() 
        }
    }
}

impl<S> TimedRun<S>
where 
    S: Split {

    pub fn new(name: LevelDescriptor, player_count: u8) -> Self {
        Self {
            name,
            player_count,
            ..Default::default()
        }
    }

    pub fn add_split(&mut self, split: S) {
        self.total_time += split.get_time();

        self.splits.push(split);
    }

    pub fn add_checkpoint(&mut self) {
        self.used_checkpoint = true;
    }

    pub fn add_win(&mut self) {
        self.is_win = true;
    }
    
    pub fn did_secondary(&mut self) {
        self.did_secondary = true;
    }

    pub fn did_overload(&mut self) {
        self.did_overload = true;
    }

    pub fn get_last_split(&self) -> Option<&S> {
        self.splits.last()
    }
}
