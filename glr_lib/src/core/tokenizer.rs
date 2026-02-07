use std::ops::Deref;

use glr_core::{time::Time, token::Token};

pub trait Tokenizer {
    fn tokenize_single(&self, line: &str) -> Option<Token>;
    #[allow(unused)]
    fn tokenize(&self, lines: &str) -> Vec<(Time, Token)> {
        let mut result = Vec::new();

        for line in lines.split('\n').map(|v| v.trim_start()) {
            if let Some(token) = self.tokenize_single(line) {
                if let Some(time) = Time::from(line) {
                    // #[cfg(debug_assertions)]
                    // eprintln!("{} Token parsed:{:?}", time.to_string(), token);
                    result.push((time, token));
                }
            }
        }

        result
    }
}

pub trait TokenizerGetIter: Tokenizer {
    fn tokenize_to_iter(&self, lines: &str) -> impl Iterator<Item = (Time, Token)> {
        lines
            .split('\n')
            .map(|v| v.trim_start())
            .map(|v| (v, self.tokenize_single(v)))
            .filter(|(_, v)| v.is_some())
            .map(|(a, b)| (Time::from(a), b.unwrap()))
            .map(|(a, b)| (a.unwrap(), b))
    }
}

impl<T: Tokenizer> TokenizerGetIter for T {}

pub struct TokenizeIter<I, T>
where
    I: Iterator<Item = String>,
    T: Tokenizer,
{
    iter: I,
    tokenizer: T,

    last_time: Time,
    end_token_read: bool,
    is_done: bool,
}

impl<I, T> Iterator for TokenizeIter<I, T>
where
    I: Iterator<Item = String>,
    T: Tokenizer,
{
    type Item = (Time, Token);

    fn next(&mut self) -> Option<Self::Item> {
        while self.is_done == false {
            let Some(owned) = self.iter.next() else {
                self.is_done = true;
                return match self.end_token_read {
                    true => None,
                    false => Some((self.last_time, Token::LogFileEnd)),
                }
            };
            let line = owned.trim_start();

            match (self.tokenizer.tokenize_single(line), Time::from(line)) {
                (Some(token), Some(time)) => { 
                    self.end_token_read = self.end_token_read || token == Token::LogFileEnd;
                    self.last_time = time;
                    return Some((time, token))
                },
                _ => {}
            }
        }

        None
    }
}

impl<I, T> TokenizeIter<I, T>
where
    I: Iterator<Item = String>,
    T: Tokenizer,
{
    pub fn new(iter: I, tokenizer: T) -> Self {
        Self { 
            iter, 
            tokenizer, 
            end_token_read: false,
            last_time: Time::new(),
            is_done: false, 
        }
    }
}

impl Tokenizer for Box<dyn Tokenizer> {
    fn tokenize_single(&self, line: &str) -> Option<Token> {
        self.deref().tokenize_single(line)
    }
}

impl<I, T> Tokenizer for I
where
    T: Tokenizer,
    for<'a> &'a I: IntoIterator<Item = &'a T>,
{
    fn tokenize_single(&self, line: &str) -> Option<Token> {
        self.into_iter().find_map(|v| v.tokenize_single(line))
    }
}

#[derive(Default)]
pub struct BaseTokenizer;
#[derive(Default)]
pub struct RunTokenizer;
#[derive(Default)]
pub struct GenerationTokenizer;
#[derive(Default)]
pub struct CheckpointTokenizer;
#[derive(Default)]
pub struct AllTokenizer;

fn check_match(line: &str, start_id: usize, search: &str) -> bool {
    line.get(start_id..(start_id + search.len()))
        .is_some_and(|v| v == search)
}

impl Tokenizer for BaseTokenizer {
    fn tokenize_single(&self, line: &str) -> Option<Token> {
        if check_match(line, 44, "SetSessionIDSeed") {
            return Some(Token::create_session_seed(line));
        }
        if check_match(line, 29, "PlayFab.OnGetCurrentTime") {
            return Some(Token::create_utc_time(line));
        }
        if check_match(line, 30, "SelectActiveExpedition") {
            return Some(Token::create_expedition(line));
        }
        if check_match(line, 15, "OnApplicationQuit") {
            return Some(Token::LogFileEnd);
        }

        let len = line.len();

        if check_match(line, len.saturating_sub(21), "was added to session") {
            return Some(Token::create_player_joined(line));
        }
        if check_match(line, 15, "<color=green>SNET : Player") {
            return Some(Token::create_player_exit_elevator(line));
        }
        if check_match(line, 15, "DEBUG : Closed connection with") {
            return Some(Token::create_player_left(line));
        }
        if check_match(line, 15, "DEBUG : Leaving session hub!") {
            return Some(Token::UserExitLobby);
        }
        if check_match(line, 15, "Player Down") {
            return Some(Token::create_player_down(line));
        }

        None
    }
}

impl Tokenizer for RunTokenizer {
    fn tokenize_single(&self, line: &str) -> Option<Token> {
        if line.contains("exits PLOC_InElevator") {
            return Some(Token::create_player(line));
        }
        if check_match(line, 69, ": StopElevatorRide TO: ReadyToStartLevel") {
            return Some(Token::GameStarting);
        }
        if check_match(line, 69, ": ReadyToStartLevel TO: InLevel") {
            return Some(Token::GameStarted);
        }
        if check_match(line, 31, "LinkedToZoneData.EventsOnEnter") {
            return Some(Token::DoorOpen);
        }
        if check_match(line, 15, "BulkheadDoorController_Core") {
            return Some(Token::BulkheadScanDone);
        }
        if check_match(line, 116, "WardenObjectiveItemSolved") {
            return Some(Token::SecondaryDone);
        }
        if check_match(line, 112, "WardenObjectiveItemSolved") {
            return Some(Token::OverloadDone);
        }
        if check_match(line, 71, "InLevel TO: ExpeditionSuccess") {
            return Some(Token::GameEndWin);
        }
        if check_match(line, 15, "RundownManager.OnExpeditionEnded(endState: Abort") {
            return Some(Token::GameEndAbort);
        }
        if check_match(line, 15, "CleanupAfterExpedition AfterLevel") {
            return Some(Token::GameEndAbort);
        }
        if check_match(line, 15, "DEBUG : Leaving session hub! : IsInHub:True") {
            return Some(Token::GameEndAbort);
        }
        if check_match(line, 71, "InLevel TO: ExpeditionFail") {
            return Some(Token::GameEndLost);
        }

        None
    }
}

impl Tokenizer for CheckpointTokenizer {
    fn tokenize_single(&self, line: &str) -> Option<Token> {
        if check_match(line, 71, "ExpeditionFail TO: InLevel") {
            return Some(Token::CheckpointReset);
        }

        None
    }
}

impl Tokenizer for GenerationTokenizer {
    fn tokenize_single(&self, line: &str) -> Option<Token> {
        if check_match(line, 69, ": Lobby TO: Generating") {
            return Some(Token::GeneratingLevel);
        }
        if check_match(line, 69, ": Generating TO: ReadyToStopElevatorRide") {
            return Some(Token::GeneratingFinished);
        }
        if check_match(line, 29, "CreateKeyItemDistribution") {
            return Some(Token::create_item_alloc(line));
        }
        if check_match(line, 30, "TryGetExistingGenericFunctionDistributionForSession") {
            return Some(Token::create_item_spawn(line));
        }
        if check_match(line, 30, "LG_Distribute_WardenObjective.SelectZoneFromPlacementAndKeepTrackOnCount") {
            return Some(Token::create_collectable_allocated(line));
        }
        if check_match(line, 35, "TryGetRandomPlacementZone.  Determine wardenobjective zone. Found zone with LocalIndex") {
            return Some(Token::create_hsu_alloc(line));
        }
        if check_match(line, 35, "LG_Distribute_WardenObjective, placing warden objective item with function") {
            return Some(Token::create_objective_spawned_override(line));
        }
        if check_match(line, 30, "LG_Distribute_WardenObjective.DistributeGatherRetrieveItems") {
            return Some(Token::create_collectable_item_id(line));
        }
        if check_match(line, 15, "GenericSmallPickupItem_Core.SetupFromLevelgen, seed:") {
            return Some(Token::create_collectable_item_seed(line));
        }
        if check_match(line, 15, "RESET placementDataIndex to 0") {
            return Some(Token::DimensionReset);
        }
        if check_match(line, 15, "Increment placementDataIndex to ") {
            return Some(Token::DimensionIncrease);
        }

        None
    }
}

impl Tokenizer for AllTokenizer {
    fn tokenize_single(&self, line: &str) -> Option<Token> {
        BaseTokenizer.tokenize_single(line)
            .or_else(|| RunTokenizer.tokenize_single(line))
            .or_else(|| GenerationTokenizer.tokenize_single(line))
            .or_else(|| CheckpointTokenizer.tokenize_single(line))
    }
}

#[cfg(test)]
mod tests {
    use std::{env, fs::File, io::Read};
    use glr_core::data::ObjectiveFunction;

    use super::*;

    fn create_tokenizer() -> AllTokenizer {
        AllTokenizer
    }

    fn load_file(name: &str) -> Option<String> {
        let mut result = String::default();
        let path_buf = env::current_dir()
            .ok()?
            .parent()?
            .join("examples")
            .join("log_files")
            .join(name)
            .with_extension("txt");

        println!("{:?}", path_buf);

        let mut f = File::open(path_buf).ok()?;

        match f.read_to_string(&mut result) {
            Ok(_) => Some(result),
            Err(_) => None,
        }
    }

    fn tokenize_file(name: &str, tokenizer: &AllTokenizer) -> Vec<Token> {
        let file_str = load_file(name).unwrap();

        tokenizer
            .tokenize(&file_str)
            .into_iter()
            .filter_map(|(_, v)| match v {
                Token::PlayerJoinedLobby(_)
                | Token::PlayerLeftLobby(_)
                | Token::UserExitLobby
                | Token::SessionSeed(_)
                | Token::PlayerDroppedInLevel(_)
                | Token::SelectExpedition(_, _)
                | Token::LogFileEnd => None,
                _ => Some(v),
            })
            .collect()
    }

    #[test]
    fn test_generation_r1a1() {
        let tokenizer = create_tokenizer();

        let tokens_v = vec![
            tokenize_file("R1A1_client.frosty_exp_comp.txt", &tokenizer),
            tokenize_file("R1A1_host.maid_exp_comp.txt", &tokenizer),
        ];

        for tokens in tokens_v {
            assert_eq!(
                tokens,
                vec![
                    Token::GeneratingLevel,
                    Token::ItemAllocated("KEY_GREEN_245".try_into().unwrap()),
                    Token::ItemSpawn(50, 48),
                    Token::CollectableAllocated(3),
                    Token::ObjectiveSpawnedOverride(18, ObjectiveFunction::HSU_FindTakeSample),
                    Token::GeneratingFinished,
                    Token::GameStarting,
                    Token::GameStarted,
                    Token::DoorOpen,
                    Token::DoorOpen,
                    Token::GameEndWin,
                    Token::GameEndAbort,
                ]
            );
        }
    }

    #[test]
    fn test_run_r1b1() {
        let tokenizer = create_tokenizer();

        let tokens_v = vec![
            tokenize_file("R1B2_client.alex_hsu_3keys.txt", &tokenizer),
            tokenize_file("R1B2_client.frosty_hsu_3keys.txt", &tokenizer),
            tokenize_file("R1B2_host.maid_hsu_3keys.txt", &tokenizer),
        ];

        for tokens in tokens_v {
            assert_eq!(
                tokens,
                vec![
                    Token::GeneratingLevel,
                    Token::ItemAllocated("KEY_BLUE_184".try_into().unwrap()),
                    Token::ItemSpawn(18, 8),
                    Token::ItemAllocated("KEY_PURPLE_421".try_into().unwrap()),
                    Token::ItemSpawn(23, 20),
                    Token::ItemAllocated("KEY_YELLOW_990".try_into().unwrap()),
                    Token::ItemSpawn(23, 37),
                    Token::CollectableAllocated(5),
                    Token::ObjectiveSpawnedOverride(16, ObjectiveFunction::HSU_FindTakeSample),
                    Token::GeneratingFinished,
                    Token::GameStarting,
                    Token::GameStarted,
                    Token::DoorOpen,
                    Token::DoorOpen,
                    Token::DoorOpen,
                    Token::DoorOpen,
                    Token::GameEndLost,
                    Token::GameEndAbort,
                    Token::GeneratingLevel,
                    Token::ItemAllocated("KEY_PURPLE_389".try_into().unwrap()),
                    Token::ItemSpawn(18, 1),
                    Token::ItemAllocated("KEY_GREY_560".try_into().unwrap()),
                    Token::ItemSpawn(23, 21),
                    Token::ItemAllocated("KEY_ORANGE_338".try_into().unwrap()),
                    Token::ItemSpawn(22, 14),
                    Token::CollectableAllocated(5),
                    Token::ObjectiveSpawnedOverride(16, ObjectiveFunction::HSU_FindTakeSample),
                    Token::GeneratingFinished,
                    Token::GameStarting,
                    Token::GameStarted,
                    Token::DoorOpen,
                    Token::DoorOpen,
                    Token::DoorOpen,
                    Token::DoorOpen,
                    Token::GameEndWin,
                    Token::GameEndAbort,
                ]
            );
        }
    }

    #[test]
    fn test_r6c2() {
        let tokenizer = create_tokenizer();

        let tokens = tokenize_file("R6C2_host_hisec.txt", &tokenizer);

        assert_eq!(
            tokens,
            vec![
                Token::GeneratingLevel,
                Token::ItemAllocated("BULKHEAD_KEY_538".try_into().unwrap()),
                Token::ItemSpawn(123, 1),
                Token::ItemAllocated("BULKHEAD_KEY_585".try_into().unwrap()),
                Token::ItemSpawn(247, 5),
                Token::CollectableAllocated(246),
                Token::CollectableItemID(154),
                Token::DimensionReset,
                Token::GeneratingFinished,
                Token::GameStarting,
                Token::GameStarted,
                Token::DoorOpen,
                Token::GameEndAbort,
                Token::GameEndAbort,
            ]
        );
    }
}
