use std::error::Error;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use regex::Regex;
use serde::{Serialize, Deserialize};

use super::data::{KeyDescriptor, LevelDescriptor, ObjectiveFunction, Rundown};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Token {
    GeneratingLevel,

    PlayerJoinedLobby(String),
    PlayerLeftLobby(String),
    PlayerDown(String),
    PlayerExitElevator(String),
    UserExitLobby,

    TimeSessionStart(DateTime<Utc>),
    SessionSeed(u64),
    GeneratingFinished,
    ItemAllocated(KeyDescriptor),                     // name
    ItemSpawn(u64, u32),                              // zone, id
    CollectableAllocated(u64),                        // zone
    ObjectiveSpawnedOverride(u64, ObjectiveFunction), // id, name of objective
    CollectableItemID(u8),                            // item id
    CollectableItemSeed(u64),                         // item seed
    DimensionIncrease,
    DimensionReset,
    SelectExpedition(LevelDescriptor, i32), // level info and seed
    GameStarting,
    GameStarted,
    PlayerDroppedInLevel(u32),
    DoorOpen,
    CheckpointReset,
    BulkheadScanDone,
    SecondaryDone,
    OverloadDone,
    GameEndWin,
    GameEndLost,
    GameEndAbort,
    LogFileEnd,

    Invalid,
}

fn nth_space_index(s: &str, n: usize) -> Option<usize> {
    s.char_indices()
        .filter(|&(_, c)| c == ' ')
        .nth(n)
        .map(|(i, _)| i)
}

impl Token {
    pub fn create_utc_time(line: &str) -> Token {
        match Self::utc_time_getter(line) {
            Ok(t) => t,
            Err(_) => Token::Invalid,
        }
    }

    fn utc_time_getter(line: &str) -> Result<Token, Box<dyn Error>> {
        let re = Regex::new(
            r"(?P<h>\d{2}):(?P<m>\d{2}):(?P<s>\d{2}\.\d{3}).*?(?P<day>\d{2}) (?P<month>\w+) (?P<year>\d{4})"
        )?;

        let caps = re.captures(line).ok_or("No match found")?;

        let hour: u32 = caps["h"].parse()?;
        let minute: u32 = caps["m"].parse()?;
        let sec_ms: f64 = caps["s"].parse()?;
        let seconds = sec_ms.trunc() as u32;
        let millis = ((sec_ms.fract()) * 1000.0).round() as u32;

        let day: u32 = caps["day"].parse()?;
        let year: i32 = caps["year"].parse()?;
        let month_name = &caps["month"];

        // Map month name to number
        let month = match month_name.to_lowercase().as_str() {
            "january" => 1,
            "february" => 2,
            "march" => 3,
            "april" => 4,
            "may" => 5,
            "june" => 6,
            "july" => 7,
            "august" => 8,
            "september" => 9,
            "october" => 10,
            "november" => 11,
            "december" => 12,
            _ => return Err(format!("Unknown month: {}", month_name).into()),
        };

        // Build a UTC DateTime
        let date = NaiveDate::from_ymd_opt(year, month, day).ok_or("Invalid date")?;
        let time = NaiveTime::from_hms_milli_opt(hour, minute, seconds, millis).ok_or("Invalid time")?;
        let naive_dt = NaiveDateTime::new(date, time);
        let utc_dt: DateTime<Utc> = DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc);

        Ok(Token::TimeSessionStart(utc_dt))
    }

    pub fn create_player_joined(line: &str) -> Token {
        line
            .get(22..line.len().saturating_sub(22))
            .map(|v| Token::PlayerJoinedLobby(v.to_owned()))
            .unwrap_or_else(|| Token::Invalid)
    }

    pub fn create_player_left(line: &str) -> Token {
        line
            .get(46..line.len().saturating_sub(1))
            .map(|v| Token::PlayerLeftLobby(v.to_owned()))
            .unwrap_or_else(|| Token::Invalid)
    }

    pub fn create_player_down(line: &str) -> Token {
        line
            .get(28..line.len().saturating_sub(1))
            .map(|v| Token::PlayerDown(v.to_owned()))
            .unwrap_or_else(|| Token::Invalid)
    }

    pub fn create_player_exit_elevator(line: &str) -> Token {
        let start = nth_space_index(line, 5);
        if let None = start {
            return Token::Invalid
        }

        line
            .get(start.unwrap() + 1..line.len().saturating_sub(54))
            .map(|v| Token::PlayerExitElevator(v.to_owned()))
            .unwrap_or_else(|| Token::Invalid)
    }

    pub fn create_session_seed(line: &str) -> Token {
        let words: Vec<&str> = line.split(" ").collect();

        if words.len() < 6 {
            return Token::Invalid;
        }

        match words[5].parse::<u64>() {
            Ok(seed) => Token::SessionSeed(seed),
            Err(_) => Token::Invalid,
        }
    }

    pub fn create_item_alloc(line: &str) -> Token {
        let words: Vec<&str> = line.split(" ").collect();

        if words.len() < 6 {
            return Token::Invalid;
        }

        let name = words[5].try_into();

        match name {
            Ok(key) => Token::ItemAllocated(key),
            Err(_) => Token::Invalid,
        }
    }

    pub fn create_item_spawn(line: &str) -> Token {
        let words: Vec<&str> = line.split(" ").collect();

        if words.len() < 15 {
            return Token::Invalid;
        }
        if words[6].len() < 4 {
            return Token::Invalid;
        }

        let zone = words[6][4..].parse().ok();
        let id = words[14].parse::<u32>();

        match (zone, id) {
            (Some(zone), Ok(id)) => Token::ItemSpawn(zone, id),
            _ => Token::Invalid,
        }
    }

    pub fn create_collectable_allocated(line: &str) -> Token {
        let words: Vec<&str> = line.split(" ").collect();

        if words.len() < 8 {
            return Token::Invalid;
        }
        if words[7].len() < 4 {
            return Token::Invalid;
        }

        match words[7][4..].parse() {
            Ok(zone) => Token::CollectableAllocated(zone),
            Err(_) => Token::Invalid,
        }
    }

    pub fn create_objective_spawned_override(line: &str) -> Token {
        let words: Vec<&str> = line.split(" ").collect();

        if words.len() < 19 {
            return Token::Invalid;
        }

        let name = words[13].into();

        if let Some(first) = words[18].split('_').nth(0) {
            match first.parse::<u64>() {
                Ok(i) => return Token::ObjectiveSpawnedOverride(i, name),
                Err(_) => return Token::Invalid,
            }
        }

        Token::Invalid
    }

    pub fn create_hsu_alloc(line: &str) -> Token {
        let words: Vec<&str> = line.split(" ").collect();

        if words.len() < 13 {
            return Token::Invalid;
        }
        if words[12].len() < 5 {
            return Token::Invalid;
        }

        match words[12][5..words[12].len() - 1].parse() {
            Ok(zone) => Token::CollectableAllocated(zone),
            Err(_) => Token::Invalid,
        }
    }

    pub fn create_collectable_item_id(line: &str) -> Token {
        let words: Vec<&str> = line.split(" ").collect();

        if words.len() < 9 {
            return Token::Invalid;
        }

        match words[8].parse() {
            Ok(id) => Token::CollectableItemID(id),
            Err(_) => Token::Invalid,
        }
    }

    pub fn create_collectable_item_seed(line: &str) -> Token {
        let words: Vec<&str> = line.split(" ").collect();

        if words.len() < 4 {
            return Token::Invalid;
        }

        match words[4].parse() {
            Ok(seed) => Token::CollectableItemSeed(seed),
            Err(_) => Token::Invalid,
        }
    }

    pub fn create_expedition(line: &str) -> Token {
        //println!("LINE: {}", line);

        let words: Vec<&str> = line.split(" ").collect();

        if words.len() < 11 {
            return Token::Invalid;
        }
        if words[6].len() < 6 {
            return Token::Invalid;
        }
        if words[7].len() < 5 {
            return Token::Invalid;
        }

        let rundown_id = &words[6][6..];
        let tier = match words[7].bytes().nth(4) {
            Some(val) => val - 'A' as u8,
            None => return Token::Invalid,
        }
        .into();
        let level = match words[8].parse::<u8>() {
            Ok(val) => val,
            Err(_) => return Token::Invalid,
        }
        .into();
        let seed = match words[10].parse::<i32>() {
            Ok(val) => val,
            Err(_) => return Token::Invalid,
        }
        .into();

        let rundown: Rundown = rundown_id.parse::<u8>().unwrap_or_default().into();

        Token::SelectExpedition(LevelDescriptor::new(rundown, tier, level), seed)
    }

    pub fn create_player(line: &str) -> Token {
        let words: Vec<&str> = line.split(" ").collect();

        let player_id = words[words.len() - 1].trim();
        let player_id = &player_id[0..player_id.len() - 8];

        match player_id.parse::<u32>() {
            Ok(id) => Token::PlayerDroppedInLevel(id),
            Err(_) => Token::Invalid,
        }
    }
}
