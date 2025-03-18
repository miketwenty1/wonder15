use std::collections::HashMap;

use bevy::{color::Color, ecs::resource::Resource};
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
pub struct BlockchainDataHeight(pub u32);

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
#[derive(Resource, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldBlockchainTileMap {
    pub map: HashMap<u32, TileBlockchainData>,
}

#[derive(Resource, Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct TileData {
    pub ln_address: Option<String>,
    pub username: Option<String>,
    pub color: Color,
    pub message: Option<String>,
    pub value: u32,
    pub cost: u32,
    pub height: u32,
    pub land_index: u32,
    pub event_date: Option<DateTime<Utc>>,
    pub resource: TileResource,
    pub block_hash: [u8; 32],
}

#[derive(Resource, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct TileBlockchainData {
    pub height: u32,
    pub block_hash: [u8; 32],
    pub block_time: u64,
    pub block_bits: u32,
    pub block_n_tx: u32,
    pub block_size: u32,
    pub block_fee: u64,
    pub block_weight: u64,
    pub block_ver: u32,
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
