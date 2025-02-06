use bevy::prelude::*;
use channels::init_js_comms_channels;
use chrono::{Duration, Utc};
use ecs::{
    event::RequestTileUpdates,
    resource::{ApiPollingTimer, UpdateGameTimetamp},
};
//use request_game_tiles::{api_get_server_tiles, api_receive_server_tiles};
use timer::tick_api_receive_timer;

mod channels;
mod ecs;
mod timer;

pub struct CommsPlugin;

impl Plugin for CommsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ApiPollingTimer>()
            .add_event::<RequestTileUpdates>()
            .add_systems(Startup, init_js_comms_channels)
            .add_systems(
                Update,
                (
                    tick_api_receive_timer,
                    // api_get_server_tiles,
                    // api_receive_server_tiles,
                ),
            )
            .insert_resource(UpdateGameTimetamp {
                ts: Utc::now() - Duration::minutes(5),
            });
    }
}
