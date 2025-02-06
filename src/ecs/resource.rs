use bevy::{color::Color, ecs::system::Resource, utils::HashMap};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Resource, Clone)]
pub struct GameStaticInputs {
    pub username: String,
    pub ln_address: String,
    pub using_iphone: bool,
    pub server_url: String,
    pub blockchain_filters: bool,
    pub full_map_mode: bool,
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct BlockchainHeight(pub u32);

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct WinSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct GameHeight(pub u32);

#[derive(Resource, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldOwnedTileMap {
    pub map: HashMap<u32, TileData>,
}

#[derive(Resource, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct TileData {
    pub ln_address: String,
    pub username: String,
    pub color: Color,
    pub message: String,
    pub value: u32,
    pub cost: u32,
    pub height: u32,
    pub land_index: usize,
    pub event_date: DateTime<Utc>,
    pub resource: TileResource,
    pub block_hash: String,
    pub block_time: i64,
    pub block_bits: i64,
    pub block_n_tx: i32,
    pub block_size: i32,
    pub block_fee: i64,
    pub block_weight: i64,
    pub block_ver: i32,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub enum TileResource {
    Mountain,
    Water,
    Grass,
    Forest,
    Desert,
    #[default]
    Unknown,
}
impl TileResource {
    pub fn spritesheet_index_value(&self) -> usize {
        match self {
            TileResource::Mountain => 0,
            TileResource::Water => 1,
            TileResource::Grass => 2,
            TileResource::Forest => 3,
            TileResource::Desert => 4,
            TileResource::Unknown => 35,
        }
    }
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct FullMapLength(pub u32);
