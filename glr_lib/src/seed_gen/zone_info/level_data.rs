use glr_core::seed_indexer_result::{OutputSeedIndexer, ResourceType};
use serde::{Deserialize, Serialize};

use crate::{
    dll_exports::callback_handler::HasCallbackHandler, 
    output_trait::OutputTrait, 
    seed_gen::{
        consumers::base_consumer::Consumer, 
        zone_info::{
            generated_data::{grab_spawn_id, AllocType, GeneratedZone}, objective::{StagedObjective, StagedObjectiveEnum, Task}, unlock_method::{UnlockMethodType, ZoneLocationSpawn}, zone_data::{ContainerOrWorldspawn, ZoneData}
        }
    }
};


#[derive(Debug, Serialize, Deserialize)]
pub struct LevelData {

    pub skip_start: usize,

    pub zones: Vec<ZoneData>,
    #[serde(default)] pub bulk_keys_main: Vec<Vec<ZoneLocationSpawn>>,
    #[serde(default)] pub bulk_keys_sec: Vec<Vec<ZoneLocationSpawn>>,
    #[serde(default)] pub bulk_keys_ovrl: Vec<Vec<ZoneLocationSpawn>>,
    pub staged_objectives: Vec<StagedObjectiveEnum>,
    
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
            let (name, useless_seeds) = match key.unlock_type {
                UnlockMethodType::None => ("Unknown", 0usize),
                UnlockMethodType::Cell => ("Cell", 0),
                UnlockMethodType::ColoredKey => ("ColoredKey", 2),
                UnlockMethodType::BulkheadKey => ("BulkKey", 1),
            };
            let zone = key.grab_zone(seed_iter.nth(useless_seeds)?);
            let id = grab_spawn_id(
                generated_zones, 
                zone, 
                (&key.unlock_type).try_into().ok()?, 
                seed_iter.next()?,
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
            ResourceType::Ammopack => (zone.ammo_weights, zone.ammo * 0.8f32),
            ResourceType::ToolRefillpack => (zone.tool_weights, zone.tool * 0.7f32),
        };

        if left == 0.0 {
            return Some(())
        }

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
                seed_iter.next()?,
            )?;

            let (l, pack_size) = if take_seed < 0.333333f32 {
                Self::try_remove(left, 0.6f32)
            } else if take_seed < 0.6666666f32 {
                Self::try_remove(left, 1.0f32)
            } else {
                Self::try_remove(left, 0.4f32)
            };

            if l <= 0.2f32 && l > 0f32 {
                output.output(OutputSeedIndexer::ResourcePack(
                    res_type, zone.zone_id.zone_id, id as i32, pack_size + 1
                ));
            } else {
                output.output(OutputSeedIndexer::ResourcePack(
                    res_type, zone.zone_id.zone_id, id as i32, pack_size + 1
                ));
            }

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
        for val in &zone.artifacts {
            let id = grab_spawn_id(
                generated_zones, 
                &ZoneLocationSpawn { 
                    zone_id: zone.zone_id, 
                    start_weight: 0,    
                    middle_weight: 0, 
                    end_weight: 0 
                }, 
                val.into(), 
                seed_iter.next()?,
            )?;

            let name = match val {
                ContainerOrWorldspawn::Container => "ArtifactContainer",
                ContainerOrWorldspawn::Worldspawn => "ArtifactWorldspawn",
            };

            output.output(OutputSeedIndexer::Key(name.to_owned(), zone.zone_id.zone_id, id as i32));
        }

        for val in &zone.consumables {
            let id = grab_spawn_id(
                generated_zones, 
                &ZoneLocationSpawn { 
                    zone_id: zone.zone_id, 
                    start_weight: 0,    
                    middle_weight: 0, 
                    end_weight: 0 
                }, 
                val.into(), 
                seed_iter.next()?,
            )?;

            let name = match val {
                ContainerOrWorldspawn::Container => {
                    let _ = seed_iter.next()?;
                    "ConsumableContainer"
                },
                ContainerOrWorldspawn::Worldspawn => "ConsumableWorldspawn",
            };

            output.output(OutputSeedIndexer::Key(name.to_owned(), zone.zone_id.zone_id, id as i32));
        }

        Some(())
    }

    fn do_big_pickus<O: HasCallbackHandler>(
        generated_zones: &mut Vec<GeneratedZone>,
        zone: &ZoneData,
        seed_iter: &mut dyn Iterator<Item = f32>,
        output: &mut O,
    ) -> Option<()> {
        for pickup in &zone.big_pickups {
            let id = grab_spawn_id(
                generated_zones, 
                &ZoneLocationSpawn { 
                    zone_id: zone.zone_id, 
                    start_weight: pickup.start_weight, 
                    middle_weight: pickup.middle_weight, 
                    end_weight: pickup.end_weight 
                }, 
                AllocType::BigPickup, 
                seed_iter.next()?
            )?;

            output.output(OutputSeedIndexer::Key(pickup.name.clone(), zone.zone_id.zone_id, id as i32));
        }

        Some(())
    }

    fn do_layer_cells<O: HasCallbackHandler>(
        &self, 
        generated_zones: &mut Vec<GeneratedZone>, 
        layer: u8, 
        seed_iter: &mut dyn Iterator<Item = f32>, 
        output: &mut O,
    ) -> Option<()> {
        let cells = self.zones
            .iter()
            .filter_map(|v| {
                if v.unlocked_by.zones.get(0)?.zone_id.layer_id != layer { return None }

                match v.unlocked_by.unlock_type {
                    UnlockMethodType::Cell => Some(&v.unlocked_by),
                    _ => None
                }
            });

        for cell in cells {
            let zone = cell.grab_zone(seed_iter.next()?);
            let id = grab_spawn_id(
                generated_zones, 
                zone, 
                (&cell.unlock_type).try_into().ok()?, 
                seed_iter.next()?,
            )?;
            
            output.output(OutputSeedIndexer::Key("Cell".to_owned(), zone.zone_id.zone_id, id as i32))
        }

        Some(())
    }

    fn do_layer<O: HasCallbackHandler>(
        &self, 
        generated_zones: &mut Vec<GeneratedZone>,
        layer: u8, 
        seed_iter: &mut dyn Iterator<Item = f32>, 
        output: &mut O,
    ) -> Option<Task<O>> {
            self.do_layer_keys(generated_zones, layer, seed_iter, output)?;

            for zone in self.zones.iter()
                .filter(|v| v.zone_id.dimension_id == layer) {

                Self::do_res(generated_zones, zone, seed_iter, output, ResourceType::Healthpack)?;
                Self::do_res(generated_zones, zone, seed_iter, output, ResourceType::DisinfectPack)?;
                Self::do_res(generated_zones, zone, seed_iter, output, ResourceType::Ammopack)?;
                Self::do_res(generated_zones, zone, seed_iter, output, ResourceType::ToolRefillpack)?;
            
                Self::do_consumables(generated_zones, zone, seed_iter, output)?;
                Self::do_big_pickus(generated_zones, zone, seed_iter, output)?;
            }

            Self::do_layer_cells(&self, generated_zones, layer, seed_iter, output)?;

            self.staged_objectives.get(layer as usize)
                .map(|v| v.get_task(seed_iter, output))
    }

}

impl<O> Consumer<O> for LevelData
where
    O: HasCallbackHandler {

    fn take(&self, seed_iter: &mut dyn Iterator<Item = f32>, output: &mut O) {
        let mut generated_zones: Vec<GeneratedZone> = self.zones.iter()
            .map(|v| v.into())
            .collect();

        if self.skip_start > 0 {
            let _ = seed_iter.nth(self.skip_start - 1);
        }

        vec![
            self.do_layer(&mut generated_zones, 0, seed_iter, output),
            self.do_layer(&mut generated_zones, 1, seed_iter, output),
            self.do_layer(&mut generated_zones, 2, seed_iter, output),
        ].iter()
            .filter_map(|v| v.as_ref())
            .for_each(|v| v(&mut generated_zones, seed_iter, output));
    }
}
