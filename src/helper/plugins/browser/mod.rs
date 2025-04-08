use bevy::prelude::*;

use read::{readcheck_game_tiles, request_local_storage};
use resource::{tick_browser_receive_timer, BrowserPollingTimer};
use state::BrowserStorageState;
use write::write_game_tiles;

use crate::scene::explorer::ecs::state::InitSpawnMapState;

pub struct BrowserPlugin;

pub mod event;
pub mod read;
pub mod resource;
pub mod state;
pub mod write;

impl Plugin for BrowserPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BrowserPollingTimer>()
            .add_systems(Update, write_game_tiles)
            .add_systems(
                Update,
                (
                    request_local_storage,
                    readcheck_game_tiles,
                    tick_browser_receive_timer,
                )
                    .run_if(
                        in_state(BrowserStorageState::On)
                            .and(in_state(InitSpawnMapState::Done))
                            .or(in_state(InitSpawnMapState::LocalStorageRead)),
                    ),
            );
    }
}
