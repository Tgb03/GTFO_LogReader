
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsumableConsumer {
    tracked_containers: Vec<i32>,
    total_container_count: i32,
    consumable_count: i32,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct IgnoreConsumer {
    count: usize,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum KeyType {
    #[default]
    ColoredKey,
    BulkheadKey,
    Other
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct KeyConsumer {
    key_type: KeyType,
    zones: Vec<KeyIDConsumer>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyIDConsumer {
    #[serde(default)]
    name: String,
    #[serde(default)]
    zone: i32,

    #[serde(default)] start_weight: i32,
    #[serde(default)] middle_weight: i32,
    #[serde(default)] end_weight: i32,

    spawns_per_room: Vec<i32>,
}
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
#[derive(Debug, Serialize, Deserialize)]
pub struct LevelData {
    pub skip_start: usize,
    pub zones: Vec<ZoneData>,
    #[serde(default)] pub bulk_keys_main: Vec<Vec<ZoneLocationSpawn>>,
    #[serde(default)] pub bulk_keys_sec: Vec<Vec<ZoneLocationSpawn>>,
    #[serde(default)] pub bulk_keys_ovrl: Vec<Vec<ZoneLocationSpawn>>,
    pub staged_objectives: Vec<StagedObjective>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ObjectiveConsumer {
    item_count: usize,
    max_per_zone: usize,
    objectives_in_zones: Vec<Vec<KeyIDConsumer>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneConsumer {
    zone_index: u8,
    shown_number: u32,
    medi: f32,
    disi: f32,
    ammo: f32,
    tool: f32,
    artifact_count: u32,
    consumable_in_container: u32,
    consumable_in_worldspawn: u32,
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceGeneration {
    left: f32,
    res_type: ResourceType,
    #[serde(default)] zone: i32,
    #[serde(default)] track_spawn: Option<KeyIDConsumer>,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum ResourceType {
    #[default]
    Healthpack,
    DisinfectPack,
    Ammopack,
    ToolRefillpack,
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
#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct ZoneIdentifier {
    pub layer_id: u8,
    pub dimension_id: u8,
    pub zone_id: i32,
}
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
pub enum ContainerOrWorldspawn {
    Container,
    Worldspawn,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum UnlockMethodType {
    None,
    ColoredKey,
    BulkheadKey,
    Cell,
}
fn generate_1_usize() -> usize {
    1
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnlockMethod {
    pub unlock_type: UnlockMethodType,
    #[serde(default="generate_1_usize")] pub placement_count: usize,
    pub zones: Vec<ZoneLocationSpawn>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ZoneData {
    pub zone_id: ZoneIdentifier,
    pub unlocked_by: UnlockMethod,
    pub rooms: Vec<RoomSize>,
    #[serde(default)] pub terminals: Vec<u8>,
    #[serde(default)] pub alloc_other: Vec<u8>,   
    pub medi: f32,
    #[serde(default)] pub medi_weights: [i32; 3],
    pub disi: f32,
    #[serde(default)] pub disi_weights: [i32; 3],
    pub ammo: f32,
    #[serde(default)] pub ammo_weights: [i32; 3],
    pub tool: f32,
    #[serde(default)] pub tool_weights: [i32; 3],
    pub consumables: Vec<ContainerOrWorldspawn>,
    pub artifacts: Vec<ContainerOrWorldspawn>,
    pub small_pickups: Vec<ZoneObjectSpawn>,
    pub big_pickups: Vec<ZoneObjectSpawn>,
    #[serde(default)] pub other_pickups: Vec<ZoneObjectSpawn>,
    #[serde(default)] pub allow_big_pickups: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneObjectSpawn {
    pub name: String,
    pub start_weight: i32,
    pub middle_weight: i32,
    pub end_weight: i32,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZoneLocationSpawn {
    pub zone_id: ZoneIdentifier,
    pub start_weight: i32,
    pub middle_weight: i32,
    pub end_weight: i32,
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
    let descriptors: HashMap<String, Vec<ConsumerEnum>> = serde_json::from_str(&descriptors_json).unwrap();
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
    let _: HashMap<String, Vec<ConsumerEnum>> = bincode::deserialize(&descriptors_bin).unwrap();
    let _: CollectableMapper = bincode::deserialize(&collectables_bin).unwrap();
}