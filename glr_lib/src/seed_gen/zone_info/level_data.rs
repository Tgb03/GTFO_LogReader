
use glr_core::seed_indexer_result::{OutputSeedIndexer, ResourceType};
use serde::{Deserialize, Serialize};

use crate::{
    dll_exports::callback_handler::HasCallbackHandler, 
    output_trait::OutputTrait, 
    seed_gen::{
        consumers::base_consumer::Consumer, 
        zone_info::{
            generated_data::{grab_spawn_id, AllocType, GeneratedZone}, spawn_object::SpawnObject, unlock_method::{UnlockMethodType, ZoneLocationSpawn}, zone_data::{ContainerOrWorldspawn, ZoneData}
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
    pub staged_objectives: Vec<StagedObjective>,
    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StagedObjective {

    pub locations: Vec<Vec<ZoneLocationSpawn>>,
    pub name: String,
    pub spawn_type: Option<AllocType>,
    pub count: usize,
    pub max_per_zone: usize,
    pub spawn_in_layer: bool,

}

impl StagedObjective {

    fn get_task<O: HasCallbackHandler>(
        &self,
        generated_zones: &mut Vec<GeneratedZone>,
        seed_iter: &mut dyn Iterator<Item = f32>, 
        output: &mut O,
    ) -> Vec<SpawnObject> {
        let mut new_vec: Vec<Vec<(usize, ZoneLocationSpawn)>> = self.locations.iter()
            .map(|v| v.iter()
                .map(|v| (self.max_per_zone, v.clone()))
                .collect()
            )
            .collect();

        let mut result = Vec::with_capacity(self.count);
        
        for id in 0..self.count {
            let id = id % self.locations.len();
            let choices = &mut new_vec[id];

            let size = choices.len();
            let rolled_id = if self.name.as_str() != "PowerCellDistribution" {
                (seed_iter.next().unwrap() * size as f32) as usize
            } else { 0 };
            let (count, selected) = &mut choices[rolled_id];
            *count -= 1;

            let result_obj = match self.spawn_in_layer {
                true => {
                    match self.spawn_type {
                        Some(alloc_type) => { 
                            let id = grab_spawn_id(
                                generated_zones, 
                                selected, 
                                alloc_type, 
                                seed_iter.next().unwrap()
                            ).unwrap();

                            output.output(OutputSeedIndexer::Key(
                                if self.name.as_str() == "CentralGeneratorCluster" {
                                    "Cell".to_owned()
                                } else { self.name.clone() }, 
                                selected.zone_id.zone_id, 
                                id as i32
                            ));
                        },
                        None => {},
                    };

                    None
                },
                false => match self.spawn_type {
                    Some(sp_t) => Some(SpawnObject {
                        name: if self.name.as_str() == "CentralGeneratorCluster" {
                            "Cell".to_owned()
                        } else { self.name.clone() },
                        start_weight: selected.start_weight,
                        middle_weight: selected.middle_weight,
                        end_weight: selected.end_weight,
                        alloc_type: sp_t,
                        zone_id: selected.zone_id,
                    }),
                    None => None,
                }
            };

            if let Some(result_obj) = result_obj {
                result.push(result_obj);
            }

            choices.retain(|v| v.0 > 0);
        }

        // if self.name.as_str() == "GatherSmallItems" {
        //     result.sort_by_key(|v| v.zone_id.zone_id);
        // }

        result
    }

}

impl LevelData {

    fn do_layer_keys<O: HasCallbackHandler>(
        &self, 
        generated_zones: &mut Vec<GeneratedZone>, 
        layer: u8, 
        seed_iter: &mut dyn Iterator<Item = f32>, 
        output: &mut O,
    ) -> Option<Vec<ZoneLocationSpawn>> {
        let result = self.zones
            .iter()
            .filter_map(|v| {
                if v.unlocked_by.zones.get(0)?.zone_id.layer_id != layer { return None }

                match v.unlocked_by.unlock_type {
                    UnlockMethodType::None => None,
                    _ => Some(&v.unlocked_by),
                }
            })
            .filter_map(|key| {
                let (name, useless_seeds) = match key.unlock_type {
                    UnlockMethodType::None => ("Unknown", 0usize),
                    UnlockMethodType::Cell => {
                        println!("got cell");
                        return Some((0..key.placement_count).into_iter()
                            .map(|_| key.grab_zone(seed_iter.next().unwrap()))
                            .collect::<Vec<&ZoneLocationSpawn>>())
                    },
                    UnlockMethodType::ColoredKey => ("ColoredKey", 2),
                    UnlockMethodType::BulkheadKey => ("BulkKey", 1),
                };
                println!("Got key");
                let zone = key.grab_zone(seed_iter.nth(useless_seeds)?);
                let id = grab_spawn_id(
                    generated_zones, 
                    zone, 
                    (&key.unlock_type).try_into().ok()?, 
                    seed_iter.next()?,
                )?;

                output.output(OutputSeedIndexer::Key(name.to_owned(), zone.zone_id.zone_id, id as i32));

                None
            })
            .fold(Vec::new(), |mut v, a| {
                v.extend(a.into_iter().cloned());
                v
            });

        match layer {
            0 => &self.bulk_keys_main,
            1 => &self.bulk_keys_sec,
            _ => &self.bulk_keys_ovrl,
        }.iter()
            .for_each(|v| {
                let zone = &v[
                    (seed_iter.nth(1).unwrap() * v.len() as f32) as usize
                ];
                let id = grab_spawn_id(
                    generated_zones, 
                    zone, 
                    AllocType::Container, 
                    seed_iter.next().unwrap(),
                ).unwrap_or_default();

                println!("Got bulk key");
                output.output(OutputSeedIndexer::Key("BulkKey".to_owned(), zone.zone_id.zone_id, id as i32));
            });

        Some(result)
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
            let seed = seed_iter.next()?;
            let id = grab_spawn_id(
                generated_zones, 
                &ZoneLocationSpawn { 
                    zone_id: zone.zone_id, 
                    start_weight: pickup.start_weight, 
                    middle_weight: pickup.middle_weight, 
                    end_weight: pickup.end_weight 
                }, 
                AllocType::BigPickup, 
                seed,
            )?;

            output.output(OutputSeedIndexer::Key(pickup.name.clone(), zone.zone_id.zone_id, id as i32));
        }

        Some(())
    }

    fn do_other_pickups<O: HasCallbackHandler>(
        generated_zones: &mut Vec<GeneratedZone>,
        zone: &ZoneData,
        seed_iter: &mut dyn Iterator<Item = f32>,
        output: &mut O,
    ) -> Option<()> {
        for pickup in &zone.other_pickups {
            let id = grab_spawn_id(
                generated_zones, 
                &ZoneLocationSpawn { 
                    zone_id: zone.zone_id, 
                    start_weight: pickup.start_weight, 
                    middle_weight: pickup.middle_weight, 
                    end_weight: pickup.end_weight 
                }, 
                AllocType::Other, 
                seed_iter.next()?,
            )?;

            output.output(OutputSeedIndexer::Key(pickup.name.clone(), zone.zone_id.zone_id, id as i32));
        }

        Some(())
    }

    fn do_layer<O: HasCallbackHandler>(
        &self, 
        generated_zones: &mut Vec<GeneratedZone>,
        layer: u8, 
        seed_iter: &mut dyn Iterator<Item = f32>, 
        output: &mut O,
    ) -> Option<Vec<SpawnObject>> {
        let cell_iter = self.do_layer_keys(generated_zones, layer, seed_iter, output)?;

        for zone in self.zones.iter()
            .filter(|v| v.zone_id.layer_id == layer) {
    
            Self::do_res(generated_zones, zone, seed_iter, output, ResourceType::Healthpack)?;
            Self::do_res(generated_zones, zone, seed_iter, output, ResourceType::DisinfectPack)?;
            Self::do_res(generated_zones, zone, seed_iter, output, ResourceType::Ammopack)?;
            Self::do_res(generated_zones, zone, seed_iter, output, ResourceType::ToolRefillpack)?;
        
            Self::do_consumables(generated_zones, zone, seed_iter, output)?;
            Self::do_big_pickus(generated_zones, zone, seed_iter, output)?;
            Self::do_other_pickups(generated_zones, zone, seed_iter, output)?;
        }

        let mut vec: Vec<SpawnObject> = cell_iter.into_iter()
            .map(|v| SpawnObject {
                name: "Cell".to_owned(),
                zone_id: v.zone_id,
                start_weight: v.start_weight,
                middle_weight: v.middle_weight,
                end_weight: v.end_weight,
                alloc_type: AllocType::BigPickup,
            })
            .collect();

        if let Some(objective) = 
            self.staged_objectives.get(layer as usize)
                .map(|v| v.get_task(generated_zones, seed_iter, output)) {
            
            vec.extend(objective);
        }

        Some(vec)
        // self.do_layer_cells(generated_zones, layer, seed_iter, output)?;
        // self.do_layer_gens(layer, seed_iter);
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

        // for _ in 0..self.skip_start {
        //     println!("seed: {}", seed_iter.next().unwrap());
        // }

        let r = vec![
            self.do_layer(&mut generated_zones, 0, seed_iter, output),
            self.do_layer(&mut generated_zones, 1, seed_iter, output),
            self.do_layer(&mut generated_zones, 2, seed_iter, output),
        ];

        // self.do_layer_cells(&mut generated_zones, 0, seed_iter, output);
        // self.do_layer_cells(&mut generated_zones, 1, seed_iter, output);
        // self.do_layer_cells(&mut generated_zones, 2, seed_iter, output);

        r.into_iter()
            .filter_map(|v| v)
            .map(|v| v.into_iter())
            .fold(vec![], |mut i, v| {
                i.extend(v);
                i
            })
            .into_iter()
            .for_each(|v| { v.take(&mut generated_zones, seed_iter, output); });
    }
}
