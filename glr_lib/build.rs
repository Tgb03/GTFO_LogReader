use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelData {
    pub build_seed: i32,
    pub build_seed_gate_count: usize,
    pub skip_start: usize,
    pub zones: Vec<ZoneData>,
    #[serde(default)]
    pub bulk_keys_main: Vec<Vec<ZoneLocationSpawn>>,
    #[serde(default)]
    pub bulk_keys_sec: Vec<Vec<ZoneLocationSpawn>>,
    #[serde(default)]
    pub bulk_keys_ovrl: Vec<Vec<ZoneLocationSpawn>>,
    pub staged_objectives: Vec<StagedObjective>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZoneLocationSpawn {
    pub zone_id: ZoneIdentifier,

    pub start_weight: i32,
    pub middle_weight: i32,
    pub end_weight: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AllocType {
    Container = 0,
    SmallPickup = 1,
    BigPickup = 2,

    Terminal = 3,
    Other = 4,
}
#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct ZoneIdentifier {
    pub layer_id: u8,
    pub dimension_id: u8,
    pub zone_id: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SpawnObject {
    pub name: String,
    pub zone_id: ZoneIdentifier,

    pub start_weight: i32,
    pub middle_weight: i32,
    pub end_weight: i32,

    pub alloc_type: AllocType,
    #[serde(default)] pub skip_before_alloc: usize,
}
fn generate_1_usize() -> usize {
    1
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum UnlockMethodType {
    None,
    ColoredKey,
    BulkheadKey,
    Cell,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnlockMethod {
    pub unlock_type: UnlockMethodType,
    #[serde(default = "generate_1_usize")]
    pub placement_count: usize,
    pub zones: Vec<ZoneLocationSpawn>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ZoneData {
    pub zone_id: ZoneIdentifier,
    pub unlocked_by: UnlockMethod,
    pub rooms: Vec<RoomSize>,

    #[serde(default)]
    pub terminals: Vec<u8>,
    #[serde(default)]
    pub alloc_other: Vec<u8>,
    pub medi: f32,
    #[serde(default)]
    pub medi_weights: [i32; 3],
    pub disi: f32,
    #[serde(default)]
    pub disi_weights: [i32; 3],
    pub ammo: f32,
    #[serde(default)]
    pub ammo_weights: [i32; 3],
    pub tool: f32,
    #[serde(default)]
    pub tool_weights: [i32; 3],

    pub consumable_count: u16,
    pub artifact_count: u16,

    pub small_pickups: Vec<ZoneObjectSpawn>,
    pub big_pickups: Vec<ZoneObjectSpawn>,

    pub chance_box_consumable: f32,

    #[serde(default)] 
    pub allow_big_pickups: bool,
    #[serde(default)]
    pub allow_small_pickups: bool,
    #[serde(default)]
    pub allow_containers_alloc: bool,

    #[serde(default)]
    pub build_seed_spawners_before: u16,
    #[serde(default)]
    pub build_seed_spawners_after: u16,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum RoomSize {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Other(u8, u8, u8),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneObjectSpawn {
    pub name: String,

    pub start_weight: i32,
    pub middle_weight: i32,
    pub end_weight: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StagedObjective {
    pub locations: Vec<Vec<ZoneLocationSpawn>>,
    pub name: String,
    pub spawn_type: Option<AllocType>,
    pub count: usize,
    pub max_per_zone: usize,
    pub spawn_in_layer: bool,
    #[serde(default)] pub skip_before_alloc: usize,
}
#[derive(Serialize, Deserialize)]
pub struct CollectableMapper {
    map: HashMap<String, HashMap<u64, HashMap<u64, u64>>>,
}

fn main() {
    
    let resources_path = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("resources");
    let descriptors_json = fs::read_to_string(resources_path.join("level_descriptors.json")).unwrap();
    let collectables_json = fs::read_to_string(resources_path.join("collectable_maps.ron")).unwrap();

    // 2️⃣ Deserialize them
    let descriptors: BTreeMap<String, LevelData> = serde_json::from_str(&descriptors_json).unwrap();
    let collectables: CollectableMapper = ron::from_str(&collectables_json).unwrap();

    // 3️⃣ Serialize to binary
    let collectables_bin = bincode::serialize(&collectables).unwrap();
    let descriptors_bin = bincode::serialize(&descriptors).unwrap();

    let interop_dir = resources_path.parent()
        .unwrap()
        .join("interop");
    fs::create_dir_all(&interop_dir).expect("Failed to create interop directory");

    // 4️⃣ Write to OUT_DIR so it’s included in the build artifacts
    fs::write(interop_dir.join("level_descriptors.bin"), &descriptors_bin)
        .expect("Failed to write descriptors binary");
    fs::write(interop_dir.join("collectable_maps.bin"), &collectables_bin)
        .expect("Failed to write collectables binary");

    // checks:
    let _: BTreeMap<String, LevelData> = bincode::deserialize(&descriptors_bin).unwrap();
    let _: CollectableMapper = bincode::deserialize(&collectables_bin).unwrap();
    
}
