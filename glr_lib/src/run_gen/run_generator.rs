use std::collections::HashMap;

use glr_core::{data::LevelDescriptor, run::TimedRun, run_gen_result::RunGeneratorResult, split::{NamedSplit, Split}, time::Time, token::Token};
use regex::Regex;

fn strip_html_tags(input: &str) -> String {
    let re = Regex::new(r"<[^>]*>").unwrap();
    re.replace_all(input, "").into_owned()
}

pub struct RunGenerator<S>
where
    S: Split {

    current_run: Option<TimedRun<S>>,
    last_split_time: Time,
    last_level_name: LevelDescriptor,

    door_count: u32,
    bulk_count: u32,

    ignore_next_door: bool,
    in_death_screen: bool,

    players: HashMap<String, Time>,

}

impl<S: Split> Default for RunGenerator<S> {
    fn default() -> Self {
        Self { 
            current_run: Default::default(), 
            last_split_time: Default::default(),
            last_level_name: Default::default(), 
            door_count: Default::default(), 
            bulk_count: Default::default(),
            players: Default::default(),
            ignore_next_door: false,
            in_death_screen: false,
        }
    }
}

impl<S: Split> RunGenerator<S> {

    pub fn reset(&mut self) {
        self.current_run = None;
        self.last_split_time = Time::new();
        self.door_count = 0;
        self.bulk_count = 0;
        self.ignore_next_door = false;
        self.in_death_screen = false;
    }

}

impl RunGenerator<NamedSplit> {

    pub fn accept_token(&mut self, time: Time, token: &Token) -> Option<RunGeneratorResult> {
        match token {
            Token::SelectExpedition(level_id, _) => {
                self.last_level_name = level_id.clone();
                if let Some(run) = self.current_run.take() {
                    self.reset();
                    return Some(RunGeneratorResult::LevelRun(run));
                }
            },
            Token::PlayerJoinedLobby(name) => {
                self.players.insert(strip_html_tags(name), time);
            },
            Token::PlayerLeftLobby(name) => {
                self.players.remove(&strip_html_tags(name));
            },
            Token::PlayerDown(name) => {
                let name = strip_html_tags(name);

                if self.players
                    .get(&name)
                    .cloned()
                    .is_some_and(|t| (time - t) > Time::from_stamp(6000)) {
                    
                    self.players.get_mut(&name)
                        .map(|v| *v = time);

                    self.current_run.as_mut()
                        .map(|v| v.add_player_down(&name));
                }
            }
            Token::UserExitLobby => { 
                self.players.clear();
            },
            Token::GameStarted => { 
                self.last_split_time = time;
                self.current_run = Some(
                    TimedRun::new(
                        self.last_level_name.clone(), 
                        self.players.iter()
                            .map(|(v, _)| v.clone())
                            .collect()
                    )
                );
                
                return Some(RunGeneratorResult::GameStarted(self.last_level_name.clone(), self.players.len() as u8));
            },
            Token::DoorOpen => {
                if self.in_death_screen { return None }

                if self.ignore_next_door {
                    self.ignore_next_door = false;
                    return None
                }

                self.door_count += 1;
                let split = NamedSplit::new(
                    time - self.last_split_time, 
                    format!("D_{}", self.door_count),
                );
                self.last_split_time = time; 

                self.current_run
                    .as_mut()
                    .map(|v| v
                        .add_split(
                            split.clone()
                        )
                    );

                return Some(RunGeneratorResult::SplitAdded(split));
            },
            Token::CheckpointReset => { 
                self.current_run.as_mut().map(|v| v.add_checkpoint());
                self.ignore_next_door = true;
                self.in_death_screen = false;
                self.last_split_time = time;
                
                return Some(RunGeneratorResult::CheckpointUsed);
            },
            Token::BulkheadScanDone => {
                if self.in_death_screen { return None }

                self.bulk_count += 1;
                let split = NamedSplit::new(
                    time - self.last_split_time, 
                    format!("B_{}", self.bulk_count),
                );
                self.last_split_time = time; 
                
                self.current_run.as_mut()
                    .map(|v| v.add_split(
                        split.clone()
                    )
                );

                return Some(RunGeneratorResult::SplitAdded(split));
            },
            Token::SecondaryDone => { 
                self.current_run.as_mut().map(|v| v.did_secondary()); 
                return Some(RunGeneratorResult::SecondaryDone);
            },
            Token::OverloadDone => { 
                self.current_run.as_mut().map(|v| v.did_overload());
                return Some(RunGeneratorResult::OverloadDone); 
            },
            Token::GameEndWin => {
                self.current_run.as_mut().map(|v| v.add_win());
                let split = NamedSplit::new(
                    time - self.last_split_time, 
                    "WIN".to_owned(),
                );

                if let Some(mut run) = self.current_run.take() {
                    run.add_split(split.clone());
                    self.reset();
                return Some(RunGeneratorResult::LevelRun(run));
                }
            },
            Token::GameEndLost => {
                self.in_death_screen = true;
                let split = NamedSplit::new(
                    time - self.last_split_time, 
                    "LOSS".to_owned(),
                );

                self.current_run.as_mut()
                    .map(|r| r.add_split(
                        split
                    )
                );
            }
            Token::GameEndAbort | Token::LogFileEnd => {
                if let Some(mut run) = self.current_run.take() {
                    let split = NamedSplit::new(
                        time - self.last_split_time, 
                        "STOP".to_owned(),
                    );

                    if run.get_last_split().is_none_or(|v| v.get_name() != "LOSS") {
                        run.add_split(split);
                    }

                    self.reset();
                    return Some(RunGeneratorResult::LevelRun(run));
                }
            },
            _ => {}
        };

        None
    }

}
