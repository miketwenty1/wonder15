use std::collections::HashMap;

use bevy::{
    color::{Color, Srgba},
    ecs::resource::Resource,
    prelude::Deref,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::helper::utils::funs::get_resource_for_tile;

#[derive(Resource, Clone)]
pub struct GameStaticInputs {
    pub username: String,
    pub ln_address: String,
    pub using_iphone: bool,
    pub server_url: String,
    pub full_map_mode: bool,
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct BlockchainHeight(pub u32);

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct BlockchainFiltersHeight(pub u32);

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct WinSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct GameHeight(pub u32);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrimTile {
    // color
    pub c: Srgba,
    // value
    pub v: u32,
    // hash
    pub h: [u8; 32],
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrimGameTileForIdb {
    pub map: HashMap<u32, TrimTile>,
}

impl TrimGameTileForIdb {
    pub fn convert_trim_to_tilemap(self) -> WorldOwnedTileMap {
        let mut tile_map = HashMap::new();
        for (key, trim_tile) in self.map.into_iter() {
            let resource = get_resource_for_tile(&trim_tile.h);
            let tile_data = TileData {
                color: Color::Srgba(trim_tile.c),
                resource: resource.clone(),
                block_hash: trim_tile.h,
                value: trim_tile.v,
                cost: trim_tile.v * 2,
                height: key,
                land_index: resource.spritesheet_index_value() as u32,
                ..Default::default()
            };
            tile_map.insert(key, tile_data);
        }

        WorldOwnedTileMap { map: tile_map }
    }
}

#[derive(Resource, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldOwnedTileMap {
    pub map: HashMap<u32, TileData>,
}

impl WorldOwnedTileMap {
    pub fn trim_for_browser_storage(&self) -> TrimGameTileForIdb {
        let trimmed_map: HashMap<u32, TrimTile> = self
            .map
            .iter()
            .map(|(&key, tile_data)| {
                (
                    key,
                    TrimTile {
                        c: tile_data.color.to_srgba(),
                        v: tile_data.value,
                        h: tile_data.block_hash,
                    },
                )
            })
            .collect();

        TrimGameTileForIdb { map: trimmed_map }
    }

    pub fn to_tiledata_vec(&self) -> Vec<TileData>
    where
        TileData: Clone,
    {
        self.map.values().cloned().collect()
    }
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

#[derive(Resource, Clone, Debug, Default, Deserialize, Deref)]
pub struct FullMapLength(pub u32);

#[derive(Resource, Clone, Debug, Default, Serialize, Deserialize)]
pub struct GameTimetamp {
    pub ts: Option<DateTime<Utc>>,
}
