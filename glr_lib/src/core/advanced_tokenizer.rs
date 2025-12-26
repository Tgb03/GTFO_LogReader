use std::ops::Range;

use glr_core::token::Token;

use crate::core::tokenizer::Tokenizer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AdvancedTokenizerType {
    Base,
    Run,
    Generation,
    Checkpoint,
}

pub struct CheckMatch<'a> {
    
    range: Range<usize>,
    search_str: &'a str,
    at_start: bool,
    
}

impl<'a> CheckMatch<'a> {
    
    pub const fn new(
        search_str: &'a str,
        start_id: usize,
        is_start: bool,
    ) -> Self {
        match is_start {
            true => Self::new_start(search_str, start_id),
            false => Self::new_end(search_str, start_id),
        }
    }
    
    pub const fn new_start(
        search_str: &'a str,
        start_id: usize,
    ) -> Self {
        Self {
            range: start_id..(start_id + search_str.len()),
            search_str,
            at_start: true,
        }
    }
    
    pub const fn new_end(
        search_str: &'a str,
        start_id: usize,
    ) -> Self {
        Self {
            range: start_id..(start_id - search_str.len()),
            search_str,
            at_start: false,
        }
    }
    
    pub fn check(&self, line: &str) -> bool {
        match self.at_start {
            true => {
                line.get(self.range.clone())
                    .is_some_and(|v| v == self.search_str)
            },
            false => {
                line.get((line.len() - self.range.start)..(line.len() - self.range.end))
                    .is_some_and(|v| v == self.search_str)
            },
        }
    }
}

pub enum TokenGenerator<'a> {
    SetToken(CheckMatch<'a>, Token),
    GenToken(CheckMatch<'a>, fn(&str) -> Token),
}

impl<'a> TokenGenerator<'a> {
    pub const fn new_set_token(
        matcher: CheckMatch<'a>,
        token: Token,
    ) -> Self {
        Self::SetToken(matcher, token)
    }
    
    pub const fn new_set(
        search_str: &'a str,
        start_id: usize,
        is_start: bool,
        token: Token,
    ) -> Self {
        Self::new_set_token(CheckMatch::new(search_str, start_id, is_start), token)
    }
    
    pub const fn new_get(
        search_str: &'a str,
        start_id: usize,
        is_start: bool,
        token_gen: fn(&str) -> Token,
    ) -> Self {
        Self::new_get_token(CheckMatch::new(search_str, start_id, is_start), token_gen)
    }
    
    pub const fn new_get_token(
        matcher: CheckMatch<'a>,
        token_gen: fn(&str) -> Token
    ) -> Self {
        Self::GenToken(matcher, token_gen)
    }
}

impl<'a> Tokenizer for TokenGenerator<'a> {
    fn tokenize_single(&self, line: &str) -> Option<Token> {
        match self {
            TokenGenerator::SetToken(check_match, token) => {
                if check_match.check(line) {
                    return Some(token.clone())
                }
            },
            TokenGenerator::GenToken(check_match, f) => {
                if check_match.check(line) {
                    return Some(f(line))
                }
            },
        };
        
        None
    }
}

impl<'a> Tokenizer for [TokenGenerator<'a>] {
    fn tokenize_single(&self, line: &str) -> Option<Token> {
        self.iter()
            .filter_map(|v| v.tokenize_single(line))
            .next()
    }
}


pub const BASE_TOKENIZER: &'static [TokenGenerator<'static>] = &[
    TokenGenerator::new_get("SetSessionIDSeed", 44, true, Token::create_session_seed),
    TokenGenerator::new_get("PlayFab.OnGetCurrentTime", 29, true, Token::create_utc_time),
    TokenGenerator::new_get("SelectActiveExpedition", 30, true, Token::create_expedition),
    TokenGenerator::new_set("OnApplicationQuit", 15, true, Token::LogFileEnd),
    TokenGenerator::new_get("was added to session", 21, true, Token::create_player_joined),
    TokenGenerator::new_get("<color=green>SNET : Player", 15, true, Token::create_player_exit_elevator),
    TokenGenerator::new_get("DEBUG : Closed connection with", 15, true, Token::create_player_left),
    TokenGenerator::new_set("DEBUG : Leaving session hub!", 15, true, Token::UserExitLobby),
    TokenGenerator::new_get("Player Down", 15, true, Token::create_player_down),
];

pub const RUN_TOKENIZER: &'static [TokenGenerator<'static>] = &[
    TokenGenerator::new_get("exits PLOC_InElevator", 52, false, Token::create_player),
    TokenGenerator::new_set(": StopElevatorRide TO: ReadyToStartLevel", 69, true, Token::GameStarting),
    TokenGenerator::new_set(": ReadyToStartLevel TO: InLevel", 69, true, Token::GameStarted),
    TokenGenerator::new_set("LinkedToZoneData.EventsOnEnter", 31, true, Token::DoorOpen),
    TokenGenerator::new_set("BulkheadDoorController_Core", 15, true, Token::BulkheadScanDone),
    TokenGenerator::new_set("WardenObjectiveItemSolved", 116, true, Token::SecondaryDone),
    TokenGenerator::new_set("WardenObjectiveItemSolved", 112, true, Token::OverloadDone),
    TokenGenerator::new_set("InLevel TO: ExpeditionSuccess", 71, true, Token::GameEndWin),
    TokenGenerator::new_set("RundownManager.OnExpeditionEnded(endState: Abort", 15, true, Token::GameEndAbort),
    TokenGenerator::new_set("CleanupAfterExpedition AfterLevel", 15, true, Token::GameEndAbort),
    TokenGenerator::new_set("DEBUG : Leaving session hub! : IsInHub:True", 15, true, Token::GameEndAbort),
    TokenGenerator::new_set("InLevel TO: ExpeditionFail", 71, true, Token::GameEndLost),
];

pub const GENERATION_TOKENIZER: &'static [TokenGenerator<'static>] = &[
    TokenGenerator::new_set(": Lobby TO: Generating", 69, true, Token::GeneratingLevel),
    TokenGenerator::new_set(": Generating TO: ReadyToStopElevatorRide", 69, true, Token::GeneratingFinished),
    TokenGenerator::new_get("CreateKeyItemDistribution", 29, true, Token::create_item_alloc),
    TokenGenerator::new_get("TryGetExistingGenericFunctionDistributionForSession", 30, true, Token::create_item_spawn),
    TokenGenerator::new_get("LG_Distribute_WardenObjective.SelectZoneFromPlacementAndKeepTrackOnCount", 30, true, Token::create_collectable_allocated),
    TokenGenerator::new_get("TryGetRandomPlacementZone.  Determine wardenobjective zone. Found zone with LocalIndex", 35, true, Token::create_hsu_alloc),
    TokenGenerator::new_get("LG_Distribute_WardenObjective, placing warden objective item with function", 35, true, Token::create_objective_spawned_override),
    TokenGenerator::new_get("LG_Distribute_WardenObjective.DistributeGatherRetrieveItems", 30, true, Token::create_collectable_item_id),
    TokenGenerator::new_get("GenericSmallPickupItem_Core.SetupFromLevelgen, seed:", 15, true, Token::create_collectable_item_seed),
    TokenGenerator::new_set("RESET placementDataIndex to 0", 15, true, Token::DimensionReset),
    TokenGenerator::new_set("Increment placementDataIndex to ", 15, true, Token::DimensionIncrease),
];

pub const ALL_TOKENIZER: [TokenGenerator<'static>; 33] = [
    TokenGenerator::new_get("SetSessionIDSeed", 44, true, Token::create_session_seed),
    TokenGenerator::new_get("PlayFab.OnGetCurrentTime", 29, true, Token::create_utc_time),
    TokenGenerator::new_get("SelectActiveExpedition", 30, true, Token::create_expedition),
    TokenGenerator::new_set("OnApplicationQuit", 15, true, Token::LogFileEnd),
    TokenGenerator::new_get("was added to session", 21, true, Token::create_player_joined),
    TokenGenerator::new_get("<color=green>SNET : Player", 15, true, Token::create_player_exit_elevator),
    TokenGenerator::new_get("DEBUG : Closed connection with", 15, true, Token::create_player_left),
    TokenGenerator::new_set("DEBUG : Leaving session hub!", 15, true, Token::UserExitLobby),
    TokenGenerator::new_get("Player Down", 15, true, Token::create_player_down),
    TokenGenerator::new_get("exits PLOC_InElevator", 52, false, Token::create_player),
    TokenGenerator::new_set(": StopElevatorRide TO: ReadyToStartLevel", 69, true, Token::GameStarting),
    TokenGenerator::new_set(": ReadyToStartLevel TO: InLevel", 69, true, Token::GameStarted),
    TokenGenerator::new_set("LinkedToZoneData.EventsOnEnter", 31, true, Token::DoorOpen),
    TokenGenerator::new_set("BulkheadDoorController_Core", 15, true, Token::BulkheadScanDone),
    TokenGenerator::new_set("WardenObjectiveItemSolved", 116, true, Token::SecondaryDone),
    TokenGenerator::new_set("WardenObjectiveItemSolved", 112, true, Token::OverloadDone),
    TokenGenerator::new_set("InLevel TO: ExpeditionSuccess", 71, true, Token::GameEndWin),
    TokenGenerator::new_set("RundownManager.OnExpeditionEnded(endState: Abort", 15, true, Token::GameEndAbort),
    TokenGenerator::new_set("CleanupAfterExpedition AfterLevel", 15, true, Token::GameEndAbort),
    TokenGenerator::new_set("DEBUG : Leaving session hub! : IsInHub:True", 15, true, Token::GameEndAbort),
    TokenGenerator::new_set("InLevel TO: ExpeditionFail", 71, true, Token::GameEndLost),
    TokenGenerator::new_set(": Lobby TO: Generating", 69, true, Token::GeneratingLevel),
    TokenGenerator::new_set(": Generating TO: ReadyToStopElevatorRide", 69, true, Token::GeneratingFinished),
    TokenGenerator::new_get("CreateKeyItemDistribution", 29, true, Token::create_item_alloc),
    TokenGenerator::new_get("TryGetExistingGenericFunctionDistributionForSession", 30, true, Token::create_item_spawn),
    TokenGenerator::new_get("LG_Distribute_WardenObjective.SelectZoneFromPlacementAndKeepTrackOnCount", 30, true, Token::create_collectable_allocated),
    TokenGenerator::new_get("TryGetRandomPlacementZone.  Determine wardenobjective zone. Found zone with LocalIndex", 35, true, Token::create_hsu_alloc),
    TokenGenerator::new_get("LG_Distribute_WardenObjective, placing warden objective item with function", 35, true, Token::create_objective_spawned_override),
    TokenGenerator::new_get("LG_Distribute_WardenObjective.DistributeGatherRetrieveItems", 30, true, Token::create_collectable_item_id),
    TokenGenerator::new_get("GenericSmallPickupItem_Core.SetupFromLevelgen, seed:", 15, true, Token::create_collectable_item_seed),
    TokenGenerator::new_set("RESET placementDataIndex to 0", 15, true, Token::DimensionReset),
    TokenGenerator::new_set("Increment placementDataIndex to ", 15, true, Token::DimensionIncrease),
    TokenGenerator::new_set("ExpeditionFail TO: InLevel", 71, true, Token::CheckpointReset),
];
