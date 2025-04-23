use async_channel::{Receiver, Sender};
use bevy::{
    ecs::resource::Resource,
    time::{Timer, TimerMode},
};

use super::hard::API_POLLING_TIME;

#[derive(Resource, Clone)]
pub struct CommsChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

// #[derive(Resource, Clone)]
// pub struct RequestInvoiceChannel {
//     pub tx: Sender<String>,
//     pub rx: Receiver<String>,
// }

// #[derive(Resource, Clone)]
// pub struct CheckInvoiceChannel {
//     pub tx: Sender<String>,
//     pub rx: Receiver<String>,
// }

// #[derive(Resource, Clone)]
// pub struct UserBlockInventoryChannel {
//     pub tx: Sender<String>,
//     pub rx: Receiver<String>,
// }

// #[derive(Resource, Clone)]
// pub struct GameIDBReadChannel {
//     pub tx: Sender<String>,
//     pub rx: Receiver<String>,
// }

// #[derive(Resource, Clone)]
// pub struct BlockchainIDBReadChannel {
//     pub tx: Sender<String>,
//     pub rx: Receiver<String>,
// }
// #[derive(Resource, Clone)]
// pub struct BlockDataChannel {
//     pub tx: Sender<String>,
//     pub rx: Receiver<String>,
// }

// #[derive(Resource, Clone)]
// pub struct BlockchainTileChannel {
//     pub tx: Sender<String>,
//     pub rx: Receiver<String>,
// }

// #[derive(Resource, Clone)]
// pub struct GameTileHeightChannel {
//     pub tx: Sender<String>,
//     pub rx: Receiver<String>,
// }

// #[derive(Resource, Clone)]
// pub struct GameTileTsChannel {
//     pub tx: Sender<String>,
//     pub rx: Receiver<String>,
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
