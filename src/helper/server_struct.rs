use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::ecs::resource::TileBlockchainData;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameBlockDataFromDB {
    pub blocks: Vec<GameBlockData>,
    pub ts_checkpoint: Option<DateTime<Utc>>,
    pub height_checkpoint: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameBlockData {
    pub height: i32,
    pub event_date: DateTime<Utc>,
    pub color: String,
    pub message: String,
    pub amount: i32,
    pub username: String,
    pub refund_ln_addr: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameBlockMapData {
    pub height: i32,
    pub color: String,
    pub amount: i32,
    pub block_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameBlockMapDataHeightFromDB {
    pub blocks: Vec<GameBlockMapData>,
    pub height_checkpoint: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameBlockMapDataTSFromDB {
    pub blocks: Vec<GameBlockMapData>,
    pub ts_checkpoint: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockDataFromDB {
    pub h: i32,    // height
    pub x: String, // hash
    pub t: i64,    // time
    pub b: i64,    // bits
    pub n: i32,    // n_tx (number of transactions)
    pub s: i32,    // size
    pub f: i64,    // fee
    pub w: i64,    // weight
    pub v: i32,    // ver (version)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockchainDataHeightFromDB {
    pub blocks: Vec<BlockDataFromDB>,
    pub height_checkpoint: u32,
}
