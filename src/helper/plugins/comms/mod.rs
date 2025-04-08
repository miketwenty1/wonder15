use bevy::prelude::*;
use channels::init_js_comms_channels;
use ecs::{
    event::{GetBlockchainUpdates, RequestServerGameTiles},
    resource::ApiPollingTimer,
};
use get_blockchain_data::{api_get_blockchain_data, api_receive_blockchain_server_tiles_by_height};
use get_game_tiles_from_server::{
    api_get_map_tiles, api_receive_game_server_tiles_by_height_or_ts,
};
use timer::tick_api_receive_timer;

use crate::ecs::state::ExplorerCommsSubState;

mod channels;
pub mod ecs;
mod get_blockchain_data;
mod get_game_tiles_from_server;
mod timer;

pub struct CommsPlugin;

impl Plugin for CommsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ApiPollingTimer>()
            .add_event::<RequestServerGameTiles>()
            .add_event::<GetBlockchainUpdates>()
            .add_systems(Startup, init_js_comms_channels)
            .add_systems(Update, (tick_api_receive_timer,))
            .add_systems(
                Update,
                (
                    api_get_map_tiles,
                    api_receive_game_server_tiles_by_height_or_ts,
                )
                    .run_if(in_state(ExplorerCommsSubState::Live)),
            )
            .add_systems(
                Update,
                (
                    api_get_blockchain_data,
                    api_receive_blockchain_server_tiles_by_height,
                ),
            );
    }
}
