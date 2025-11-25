
use serde::{Deserialize, Serialize};

use crate::{
    dll_exports::callback_handler::HasCallbackHandler,
    seed_gen::{consumers::{
        base_consumer::Consumer, consumable_consumer::ConsumableConsumer, ignore_consumer::IgnoreConsumer, key_consumer::KeyConsumer, key_id_consumer::KeyIDConsumer, objective_consumer::ObjectiveConsumer, resource_generation::ResourceGeneration, zone_consumer::ZoneConsumer
    }, zone_info::level_data::LevelData},
};

pub mod base_consumer;

pub mod ignore_consumer;
pub mod key_consumer;
pub mod key_id_consumer;
pub mod resource_generation;
pub mod zone_consumer;
pub mod objective_consumer;
pub mod consumable_consumer;

#[derive(Debug, Deserialize, Serialize)]
pub enum ConsumerEnum {
    Ignore(IgnoreConsumer),
    KeyIDConsumer(KeyIDConsumer),
    ResourceGeneration(ResourceGeneration),
    KeyConsumer(KeyConsumer),
    ZoneConsumer(ZoneConsumer),
    ObjectiveConsumer(ObjectiveConsumer),
    ConsumableConsumer(ConsumableConsumer),
    LevelData(LevelData),
}

impl<O> Consumer<O> for ConsumerEnum
where
    O: HasCallbackHandler,
{
    fn take(&self, seed_iter: &mut dyn Iterator<Item = f32>, output: &mut O) {
        match self {
            ConsumerEnum::Ignore(ignore_consumer) => ignore_consumer.take(seed_iter, output),
            ConsumerEnum::KeyIDConsumer(key_idconsumer) => key_idconsumer.take(seed_iter, output),
            ConsumerEnum::ResourceGeneration(resource_generation) => resource_generation.take(seed_iter, output),
            ConsumerEnum::KeyConsumer(key_eater) => key_eater.take(seed_iter, output),
            ConsumerEnum::ZoneConsumer(zone_consumer) => zone_consumer.take(seed_iter, output),
            ConsumerEnum::ObjectiveConsumer(objective_consumer) => objective_consumer.take(seed_iter, output),
            ConsumerEnum::ConsumableConsumer(consumable_consumer) => consumable_consumer.take(seed_iter, output),
            ConsumerEnum::LevelData(level_data) => level_data.take(seed_iter, output),
        }
    }
}
