use serde::{Deserialize, Serialize};

use crate::run_gen::{run::TimedRun, split::NamedSplit};



#[derive(Debug, Serialize, Deserialize)]
pub enum RunGeneratorResult {

    GameStarted,
    SplitAdded(NamedSplit),

    SecondaryDone,
    OverloadDone,
    CheckpointUsed,

    LevelRun(TimedRun<NamedSplit>),

}
