use glr_core::seed_indexer_result::{OutputSeedIndexer, ResourceType};
use serde::{Deserialize, Serialize};

use crate::{
    dll_exports::callback_handler::HasCallbackHandler,
    output_trait::OutputTrait,
    seed_gen::consumers::{base_consumer::Consumer, key_id_consumer::KeyIDConsumer},
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceGeneration {
    left: f32,
    res_type: ResourceType,

    #[serde(default)]
    track_spawn: Option<KeyIDConsumer>,

    #[serde(skip_serializing, default)]
    seed_index: usize,
    #[serde(skip_serializing, default)]
    is_setup: bool,
    #[serde(skip_serializing, default)]
    counter: i32,
    #[serde(skip_serializing, default)]
    last_size: u8,
}

impl ResourceGeneration {
    fn setup(&mut self) {
        if self.is_setup {
            return;
        }

        self.left = match self.res_type {
            ResourceType::Ammopack => self.left * 0.8f32,
            ResourceType::ToolRefillpack => self.left * 0.7f32,
            _ => self.left,
        };

        self.is_setup = true;
    }

    fn try_remove(&mut self, value: f32) {
        self.last_size = if self.left >= 0.801f32 && value > 0.9f32 {
            5
        } else if self.left >= 0.601f32 && value >= 0.5f32 {
            4
        } else if self.left >= 0.401f32 && value >= 0.5f32 { 
            3
        } else {
            2
        };
        self.left -= value;
    }

    pub fn new(left: f32, res_type: ResourceType, track_spawn: Option<KeyIDConsumer>) -> Self {
        Self {
            left: left,
            res_type: res_type,
            track_spawn,
            ..Default::default()
        }
    }
}

impl<O> Consumer<O> for ResourceGeneration
where
    O: HasCallbackHandler,
{
    fn take(&mut self, seed: f32, output: &mut O) -> bool {
        self.setup();
        // println!("left: {}", self.left);
        
        self.seed_index += 1;

        if self.seed_index == 2 {
            if seed < 0.333333f32 {
                self.try_remove(0.6f32);
            } else if seed < 0.6666666f32 {
                self.try_remove(1.0f32);
            } else {
                self.try_remove(0.4f32);
            }
        }

        if self.seed_index == 3 {
            self.seed_index = 0;
            self.counter += 1;
        }

        if self.seed_index == 0 {
            if let Some(c_id) = self.track_spawn.as_ref() {
                let id = c_id.get_id(seed) as i32;
                
                if self.left <= 0f32 { 
                    output.output(OutputSeedIndexer::ResourcePack(
                        self.res_type, 
                        id, 
                        self.last_size
                    ));
                } else if self.left <= 0.2f32 {
                    output.output(OutputSeedIndexer::ResourcePack(
                        self.res_type, 
                        id, 
                        self.last_size + 1
                    ));
                } else {
                    output.output(OutputSeedIndexer::ResourcePack(
                        self.res_type, 
                        id, 
                        self.last_size
                    ));
                }
            }

            if self.left <= 0.2f32 {
                println!("Resource spawned: {}", self.counter);
                return true 
            }
        }

        false
    }
}
