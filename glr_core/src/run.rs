use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{data::LevelDescriptor, split::Split, time::Time};


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PlayerData {

    pub death_count: usize,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimedRun<S>
where 
    S: Split {

    name: LevelDescriptor,
    total_time: Time,
    players: HashMap<String, PlayerData>,

    used_checkpoint: u8,
    is_win: bool,

    did_secondary: bool,
    did_overload: bool,

    utc_time_started: DateTime<Utc>,

    splits: Vec<S>,

}

impl<S: Split> Default for TimedRun<S> {
    fn default() -> Self {
        Self { 
            name: Default::default(),
            total_time: Default::default(), 
            players: Default::default(), 
            used_checkpoint: Default::default(), 
            is_win: Default::default(), 
            did_secondary: Default::default(), 
            did_overload: Default::default(),
            splits: Default::default() ,
            utc_time_started: Default::default(),
        }
    }
}

impl<S> TimedRun<S>
where 
    S: Split {

    pub fn new(name: LevelDescriptor, players: Vec<String>, time: DateTime<Utc>) -> Self {
        Self {
            name,
            players: players.iter()
                .map(|p| (p.clone(), PlayerData::default()))
                .collect(),
            utc_time_started: time,
            ..Default::default()
        }
    }

    pub fn add_split(&mut self, split: S) {
        self.total_time += split.get_time();

        self.splits.push(split);
    }

    pub fn add_checkpoint(&mut self) {
        self.used_checkpoint = self.used_checkpoint.saturating_add(1);
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

    pub fn get_name(&self) -> &LevelDescriptor {
        &self.name
    }

    pub fn get_secondary(&self) -> bool {
        self.did_secondary
    }

    pub fn get_overload(&self) -> bool {
        self.did_overload
    }

    pub fn get_is_win(&self) -> bool {
        self.is_win
    }

    pub fn get_player_count(&self) -> u8 {
        self.players.len() as u8
    }

    pub fn get_players_iter(&self) -> impl Iterator<Item = &String> {
        self.players.keys().into_iter()
    }

    pub fn iter_splits(&self) -> impl Iterator<Item = &S> {
        self.splits.iter()
    }

    pub fn iter_splits_mut(&mut self) -> impl Iterator<Item = &mut S> {
        self.splits.iter_mut()
    }

    pub fn add_player_down(&mut self, name: &String) {
        self.players.get_mut(name)
            .map(|v| v.death_count += 1);
    }
}
