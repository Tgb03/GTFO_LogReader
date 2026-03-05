use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{data::LevelDescriptor, run::TimedRun, split::NamedSplit};

#[derive(Debug, Serialize, Deserialize)]
pub enum RunGeneratorResult {
    GameStarted(LevelDescriptor, u8, DateTime<Utc>),
    SplitAdded(NamedSplit),
    PlayerCountUpdate(u8),

    SecondaryDone,
    OverloadDone,
    CheckpointUsed,

    LevelRun(TimedRun<NamedSplit>),
}
