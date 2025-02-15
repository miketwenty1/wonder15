use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    pub height_checkpoint: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameBlockMapDataTSFromDB {
    pub blocks: Vec<GameBlockMapData>,
    pub ts_checkpoint: DateTime<Utc>,
}
