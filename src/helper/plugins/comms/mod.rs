use bevy::prelude::*;
use channels::init_js_comms_channels;
use chrono::{Duration, Utc};
use event::RequestTileUpdates;
use resource::UpdateGameTimetamp;
use timer::{tick_api_receive_timer, ApiPollingTimer};

mod channels;
mod event;
mod hard;
mod request_game_tiles;
pub mod resource;
mod structy;
mod timer;

pub struct CommsPlugin;

impl Plugin for CommsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ApiPollingTimer>()
            .add_event::<RequestTileUpdates>()
            .add_systems(Startup, init_js_comms_channels)
            .add_systems(Update, tick_api_receive_timer)
            .insert_resource(UpdateGameTimetamp {
                ts: Utc::now() - Duration::minutes(5),
            });
    }
}
