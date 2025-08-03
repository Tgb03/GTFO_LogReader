
use serde::{Deserialize, Serialize};

use crate::{data::LevelDescriptor, run::TimedRun, split::NamedSplit};


#[derive(Debug, Serialize, Deserialize)]
pub enum RunGeneratorResult {

    GameStarted(LevelDescriptor, u8),
    SplitAdded(NamedSplit),

    SecondaryDone,
    OverloadDone,
    CheckpointUsed,

    LevelRun(TimedRun<NamedSplit>),

}
