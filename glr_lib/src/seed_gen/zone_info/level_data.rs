use glr_core::seed_indexer_result::{OutputSeedIndexer, ResourceType};
use serde::{Deserialize, Serialize};

use crate::{
    dll_exports::callback_handler::HasCallbackHandler, 
    output_trait::OutputTrait, 
    seed_gen::{
        consumers::base_consumer::Consumer, 
        zone_info::{
            generated_data::{grab_spawn_id, AllocType, GeneratedZone}, unlock_method::{UnlockMethodType, ZoneLocationSpawn}, zone_data::ZoneData
        }
    }
};


#[derive(Debug, Serialize, Deserialize)]
pub struct LevelData {

    zones: Vec<ZoneData>,
    
}

impl LevelData {

    fn do_layer_keys<O: HasCallbackHandler>(
        &self, 
        generated_zones: &mut Vec<GeneratedZone>, 
        layer: u8, 
        seed_iter: &mut dyn Iterator<Item = f32>, 
        output: &mut O,
    ) -> Option<()> {
        let keys = self.zones
            .iter()
            .filter_map(|v| {
                if v.unlocked_by.zones.get(0)?.zone_id.layer_id != layer { return None }

                match v.unlocked_by.unlock_type {
                    UnlockMethodType::None | UnlockMethodType::Cell => None,
                    _ => Some(&v.unlocked_by),
                }
            });

        for key in keys {
            let zone = key.grab_zone(seed_iter.next().unwrap());
            let (name, useless_seeds) = match key.unlock_type {
                UnlockMethodType::None => ("Unknown", 0usize),
                UnlockMethodType::Cell => ("Cell", 0),
                UnlockMethodType::ColoredKey => ("ColoredKey", 2),
                UnlockMethodType::BulkheadKey => ("BulkKey", 1),
            };
            let id = grab_spawn_id(
                generated_zones, 
                zone, 
                (&key.unlock_type).try_into().ok()?, 
                seed_iter.nth(useless_seeds)?
            )?;

            output.output(OutputSeedIndexer::Key(name.to_owned(), zone.zone_id.zone_id, id as i32))
        }

        Some(())
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

    fn do_res<O: HasCallbackHandler>(
        generated_zones: &mut Vec<GeneratedZone>,
        zone: &ZoneData,
        seed_iter: &mut dyn Iterator<Item = f32>,
        output: &mut O,
        res_type: ResourceType,
    ) -> Option<()> {
        let (weights, mut left) = match res_type {
            ResourceType::Healthpack => (zone.medi_weights, zone.medi),
            ResourceType::DisinfectPack => (zone.disi_weights, zone.disi),
            ResourceType::Ammopack => (zone.ammo_weights, zone.ammo),
            ResourceType::ToolRefillpack => (zone.tool_weights, zone.tool),
        };

        left = left * match res_type {
            ResourceType::Ammopack => 0.8f32,
            ResourceType::ToolRefillpack => 0.7f32,
            _ => 1f32,
        };

        loop {
            let _number_seed = seed_iter.next()?;
            let take_seed = seed_iter.next()?;
            let id = grab_spawn_id(
                generated_zones,
                &ZoneLocationSpawn {
                    zone_id: zone.zone_id,
                    start_weight: weights[0],
                    middle_weight: weights[1],
                    end_weight: weights[2],
                }, 
                AllocType::Container, 
                seed_iter.next()?
            )?;

            let (l, pack_size) = Self::try_remove(left, take_seed);

            output.output(OutputSeedIndexer::ResourcePack(
                res_type, id as i32, pack_size
            ));

            left = l;

            if left <= 0.2f32 { break }
        }

        Some(())
    }

    fn do_consumables<O: HasCallbackHandler>(
        generated_zones: &mut Vec<GeneratedZone>,
        zone: &ZoneData,
        seed_iter: &mut dyn Iterator<Item = f32>,
        output: &mut O,
    ) -> Option<()> {
        for _ in 0..zone.consumables_in_containers {
            let id = grab_spawn_id(
                generated_zones, 
                &ZoneLocationSpawn { 
                    zone_id: zone.zone_id, 
                    start_weight: 0, 
                    middle_weight: 0, 
                    end_weight: 0 
                }, 
                AllocType::Container, 
                seed_iter.next()?
            )?;

            let _ = seed_iter.next();

            output.output(OutputSeedIndexer::Key("ConsumableContainer".to_owned(), zone.zone_id.zone_id, id as i32));
        }

        for _ in 0..zone.consumables_in_worldspawn {
            let id = grab_spawn_id(
                generated_zones, 
                &ZoneLocationSpawn { 
                    zone_id: zone.zone_id, 
                    start_weight: 0, 
                    middle_weight: 0, 
                    end_weight: 0 
                }, 
                AllocType::SmallPickup, 
                seed_iter.next()?,
            )?;

            output.output(OutputSeedIndexer::Key("ConsumableWorldspawn".to_owned(), zone.zone_id.zone_id, id as i32));
        }
        
        for _ in 0..zone.consumables_in_containers {
            let id = grab_spawn_id(
                generated_zones, 
                &ZoneLocationSpawn { 
                    zone_id: zone.zone_id, 
                    start_weight: 0, 
                    middle_weight: 0, 
                    end_weight: 0 
                }, 
                AllocType::Container, 
                seed_iter.next()?
            )?;

            let _ = seed_iter.next();

            output.output(OutputSeedIndexer::Key("ArtifactContainer".to_owned(), zone.zone_id.zone_id, id as i32));
        }

        for _ in 0..zone.consumables_in_worldspawn {
            let id = grab_spawn_id(
                generated_zones, 
                &ZoneLocationSpawn { 
                    zone_id: zone.zone_id, 
                    start_weight: 0, 
                    middle_weight: 0, 
                    end_weight: 0 
                }, 
                AllocType::SmallPickup, 
                seed_iter.next()?,
            )?;

            output.output(OutputSeedIndexer::Key("ArtifactWorldspawn".to_owned(), zone.zone_id.zone_id, id as i32));
        }

        Some(())
    }

}

impl<O> Consumer<O> for LevelData
where
    O: HasCallbackHandler {

    fn take(&self, seed_iter: &mut dyn Iterator<Item = f32>, output: &mut O) {
        let mut generated_zones: Vec<GeneratedZone> = self.zones.iter()
            .map(|v| v.into())
            .collect();

        if self.do_layer_keys(&mut generated_zones, 0, seed_iter, output).is_none() {
            output.output(OutputSeedIndexer::ProcessFailed);
        }

        for zone in &self.zones {
            Self::do_res(&mut generated_zones, zone, seed_iter, output, ResourceType::Healthpack);
            Self::do_res(&mut generated_zones, zone, seed_iter, output, ResourceType::DisinfectPack);
            Self::do_res(&mut generated_zones, zone, seed_iter, output, ResourceType::Ammopack);
            Self::do_res(&mut generated_zones, zone, seed_iter, output, ResourceType::ToolRefillpack);
        
            Self::do_consumables(&mut generated_zones, zone, seed_iter, output);

            
        }
    }
}
