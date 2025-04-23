use bevy::prelude::*;
use ecs::{
    event::{
        BlockDataEvent, BrowserBlockchainDataEvent, BrowserGameTilesEvent, CheckLnPaymentEvent,
        GameTilesByHeightEvent, GameTilesByTsEvent, LnInvoiceEvent, ServerBlockchainDataEvent,
        UserInventoryEvent,
    },
    resource::ApiPollingTimer,
};
use receive_comms::receive_comm;
// use get_blockchain_data::{api_get_blockchain_data, api_receive_blockchain_server_tiles_by_height};
// use get_game_tiles_from_server::{
//     api_get_map_tiles, api_receive_game_server_tiles_by_height_or_ts,
// };

pub mod ecs;
pub mod receive_comms;
// mod get_blockchain_data;
// mod get_game_tiles_from_server;
mod timer;

pub struct CommsPlugin;

impl Plugin for CommsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ApiPollingTimer>()
            // .add_event::<RequestServerGameTiles>()
            // .add_event::<GetBlockchainUpdates>()
            .add_event::<BrowserBlockchainDataEvent>()
            .add_event::<BrowserGameTilesEvent>()
            .add_event::<ServerBlockchainDataEvent>()
            .add_event::<GameTilesByTsEvent>()
            .add_event::<GameTilesByHeightEvent>()
            .add_event::<CheckLnPaymentEvent>()
            .add_event::<LnInvoiceEvent>()
            .add_event::<UserInventoryEvent>()
            .add_event::<BlockDataEvent>()
            .add_systems(Update, receive_comm)
            .add_systems(
                Update,
                (
                    log_browser_blockchain,
                    log_browser_game_tiles,
                    log_server_blockchain,
                    log_game_tiles_by_ts,
                    log_game_tiles_by_height,
                    log_check_ln_payment,
                    log_ln_invoice,
                    log_user_inventory,
                    log_block_data,
                ),
            );
        // .add_systems(
        //     Update,
        //     (
        //         api_get_map_tiles,
        //         api_receive_game_server_tiles_by_height_or_ts,
        //     )
        //         .run_if(in_state(ExplorerCommsSubState::Live)),
        // )
        // .add_systems(
        //     Update,
        //     (
        //         api_get_blockchain_data,
        //         api_receive_blockchain_server_tiles_by_height,
        //     ),
        // );
    }
}

// Dummy receivers
pub fn log_browser_blockchain(mut ev: EventReader<BrowserBlockchainDataEvent>) {
    for _ in ev.read() {
        info!("received BrowserBlockchainData");
    }
}

pub fn log_browser_game_tiles(mut ev: EventReader<BrowserGameTilesEvent>) {
    for _ in ev.read() {
        info!("received BrowserGameTiles");
    }
}

pub fn log_server_blockchain(mut ev: EventReader<ServerBlockchainDataEvent>) {
    for _ in ev.read() {
        info!("received ServerBlockchainData");
    }
}

pub fn log_game_tiles_by_ts(mut ev: EventReader<GameTilesByTsEvent>) {
    for _ in ev.read() {
        info!("received GameTilesByTs");
    }
}

pub fn log_game_tiles_by_height(mut ev: EventReader<GameTilesByHeightEvent>) {
    for _ in ev.read() {
        info!("received GameTilesByHeight");
    }
}

pub fn log_check_ln_payment(mut ev: EventReader<CheckLnPaymentEvent>) {
    for _ in ev.read() {
        info!("received CheckLnPayment");
    }
}

pub fn log_ln_invoice(mut ev: EventReader<LnInvoiceEvent>) {
    for _ in ev.read() {
        info!("received LnInvoice");
    }
}

pub fn log_user_inventory(mut ev: EventReader<UserInventoryEvent>) {
    for _ in ev.read() {
        info!("received UserInventory");
    }
}

pub fn log_block_data(mut ev: EventReader<BlockDataEvent>) {
    for _ in ev.read() {
        info!("received BlockData");
    }
}
