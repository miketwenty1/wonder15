use bevy::prelude::*;
use channels::init_js_comms_channels;
use chrono::{Duration, Utc};
use ecs::{
    event::{GetBlockchainUpdates, GetTileUpdates},
    resource::{ApiPollingTimer, UpdateGameTimetamp},
};
use get_blockchain_data::{api_get_blockchain_data, api_receive_blockchain_server_tiles_by_height};
use get_map_tiles_by_height::{api_get_map_tiles, api_receive_game_server_tiles_by_height};
use timer::tick_api_receive_timer;

use crate::ecs::state::ExplorerCommsSubState;

mod channels;
pub mod ecs;
mod get_blockchain_data;
mod get_map_tiles_by_height;
mod timer;

pub struct CommsPlugin;

impl Plugin for CommsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ApiPollingTimer>()
            .add_event::<GetTileUpdates>()
            .add_event::<GetBlockchainUpdates>()
            .add_systems(Startup, init_js_comms_channels)
            .add_systems(Update, (tick_api_receive_timer,))
            .add_systems(
                Update,
                (api_get_map_tiles, api_receive_game_server_tiles_by_height)
                    .run_if(in_state(ExplorerCommsSubState::Height)),
            )
            .add_systems(
                Update,
                (
                    api_get_blockchain_data,
                    api_receive_blockchain_server_tiles_by_height,
                ),
            )
            .insert_resource(UpdateGameTimetamp {
                ts: Utc::now() - Duration::minutes(5),
            });
    }
}
