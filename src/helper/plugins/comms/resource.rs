use async_channel::{Receiver, Sender};
use bevy::ecs::system::Resource;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// #[derive(Resource, Clone)]
// pub struct TileDataChannel {
//     pub tx: Sender<String>,
//     pub rx: Receiver<String>,
// }

#[derive(Resource, Clone)]
pub struct RequestInvoiceChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct CheckInvoiceChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct UserBlockInventoryChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

// #[derive(Resource, Clone)]
// pub struct BrowserMapLocalStorageChannel {
//     pub tx: Sender<String>,
//     pub rx: Receiver<String>,
// }

#[derive(Resource, Clone)]
pub struct CheckpointChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct BrowserIndexedDBStorageChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct BlockMessagesStorageChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct BlockchainMapChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}
#[derive(Resource, Clone)]
pub struct GameMapChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct GameTileUpdateChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct BlockchainTileUpdateChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone, Debug, Default, Serialize, Deserialize)]
pub struct UpdateGameTimetamp {
    pub ts: DateTime<Utc>,
}
