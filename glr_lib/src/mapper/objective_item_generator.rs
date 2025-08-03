use glr_core::{location::{ItemIdentifier, Location}, token::Token};

use crate::mapper::{collectable_mapper::CollectableMapper, location_generator::LocationGenerator};


pub struct ObjectiveItemGenerator {
    collectable_mapper: Option<CollectableMapper>,

    dimension: usize,
    buffer_names: Vec<ItemIdentifier>,
    buffer_zones: Vec<(usize, u64)>,

    level_name: String,
    player_count: u8,
}

impl Default for ObjectiveItemGenerator {
    fn default() -> Self {
        Self { 
            collectable_mapper: CollectableMapper::load_from_file(), 
            dimension: Default::default(), 
            buffer_names: Default::default(), 
            buffer_zones: Default::default(), 
            level_name: Default::default(),
            player_count: Default::default(),
        }
    }
}

impl LocationGenerator for ObjectiveItemGenerator {
    fn accept_token(&mut self, token: &Token) -> Option<Location> {
        match token {
            Token::PlayerJoinedLobby => {
                self.player_count += 1;
            
                None
            }
            Token::PlayerLeftLobby => {
                self.player_count -= self.player_count.saturating_sub(1);

                None
            }
            Token::UserExitLobby => {
                self.player_count = 0;

                None
            }
            Token::SelectExpedition(level_descriptor, _) => {
                self.level_name = level_descriptor.to_string();

                None
            }
            Token::CollectableAllocated(zone) => {
                self.buffer_zones.push((self.dimension, *zone));

                None
            }
            // found an item that does not have a seed
            Token::ObjectiveSpawnedOverride(id, name) => {
                // unwrap should never fail since we always know we have collectable allocated
                let (_, zone) = self.buffer_zones.pop().unwrap_or((9999, 9999));

                Some(Location::BigObjective(
                    Into::<&str>::into(name).to_owned(),
                    zone,
                    *id,
                ))
            }
            Token::CollectableItemID(id) => {
                let repr = ItemIdentifier::from_repr(*id).unwrap_or(ItemIdentifier::Unknown(*id));

                match repr {
                    ItemIdentifier::Cryo | ItemIdentifier::Cargo => {
                        // should never fail since we have collectable zone allocated
                        let (_, zone) = self.buffer_zones.remove(0);

                        Some(Location::BigCollectable(repr, zone))
                    }
                    _ => {
                        self.buffer_names.push(repr);

                        None
                    }
                }
            }
            Token::CollectableItemSeed(seed) => {
                let id = self.buffer_names.remove(0);
                if id != ItemIdentifier::DataCube && id != ItemIdentifier::DataCubeR8 {
                    self.buffer_zones.sort_by(|(d1, z1), (d2, z2)| {
                        let c = d1.cmp(d2);
                        match c {
                            std::cmp::Ordering::Equal => z1.cmp(z2),
                            _ => c,
                        }
                    });
                }
                let (_, zone) = self.buffer_zones.remove(0);

                let new_seed = self.collectable_mapper
                    .as_ref()
                    .map(|c| c.get_id(&self.level_name, zone, *seed))
                    .flatten()
                    .unwrap_or(*seed);

                Some(Location::Gatherable(id, zone, new_seed))
            }
            Token::DimensionReset => {
                // self.dimension += 1;

                None
            }
            Token::DimensionIncrease => {
                // self.dimension = 0;

                None
            }
            Token::GeneratingLevel => {
                self.buffer_names.clear();
                self.buffer_zones.clear();
                self.dimension = 0;
                
                Some(Location::GenerationStarted(format!("{}_{}", self.level_name, self.player_count)))
            }
            _ => None,
        }
    }
}

