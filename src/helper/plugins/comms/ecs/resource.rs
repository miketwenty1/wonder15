use async_channel::{Receiver, Sender};
use bevy::{
    ecs::resource::Resource,
    time::{Timer, TimerMode},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::hard::API_POLLING_TIME;
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

#[derive(Resource, Clone)]
pub struct BrowserGameCheckpointChannel {
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
pub struct GameTimetamp {
    pub ts: Option<DateTime<Utc>>,
}

// #[derive(Resource, Clone, Debug, Default, Serialize, Deserialize)]
// pub struct CheckpointTimetamp {
//     pub ts: DateTime<Utc>,
// }

#[derive(Resource)]
pub struct ApiPollingTimer {
    pub timer: Timer,
}

impl Default for ApiPollingTimer {
    fn default() -> ApiPollingTimer {
        ApiPollingTimer {
            timer: Timer::from_seconds(API_POLLING_TIME, TimerMode::Repeating),
        }
    }
}
