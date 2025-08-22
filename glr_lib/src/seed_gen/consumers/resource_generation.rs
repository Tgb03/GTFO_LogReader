
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
    #[serde(default)] zone: i32,

    #[serde(default)]
    track_spawn: Option<KeyIDConsumer>,
}

impl ResourceGeneration {
    pub fn new(left: f32, res_type: ResourceType, track_spawn: Option<KeyIDConsumer>) -> Self {
        Self {
            left: left,
            res_type: res_type,
            track_spawn,
            ..Default::default()
        }
    }

    fn try_remove(left: f32, take: f32) -> (f32, u8) {
        let ret2 = if left >= 0.0801f32 && take > 0.9f32 {
            5
        } else if left >= 0.601f32 && take >= 0.5f32 {
            4
        } else if left >= 0.401f32 && take >= 0.5f32 {
            3
        } else {
            2
        };

        (left - take, ret2)
    }
}

impl<O> Consumer<O> for ResourceGeneration
where
    O: HasCallbackHandler,
{
    fn take(&self, seed_iter: &mut dyn Iterator<Item = f32>, output: &mut O) {
        let mut left = self.left * match self.res_type {
            ResourceType::Ammopack => 0.8f32,
            ResourceType::ToolRefillpack => 0.7f32,
            _ => 1f32,
        };

        loop {
            let _number_seed = seed_iter.next().unwrap();
            let take_seed = seed_iter.next().unwrap();
            let id_seed = seed_iter.next().unwrap();

            let (l, pack_size) = if take_seed < 0.333333f32 {
                Self::try_remove(left, 0.6f32)
            } else if take_seed < 0.6666666f32 {
                Self::try_remove(left, 1.0f32)
            } else {
                Self::try_remove(left, 0.4f32)
            };

            if let Some(t_s) = &self.track_spawn {
                output.output(OutputSeedIndexer::ResourcePack(self.res_type, self.zone, t_s.get_id(id_seed) as i32, pack_size));
            }

            left = l;

            if left <= 0.2f32 { break } 
        }
    }
}
