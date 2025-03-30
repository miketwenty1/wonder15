use bevy::prelude::*;
use setup::{spawn_startup_fullmap, spawn_startup_non_fullmap};

use crate::{ecs::state::FullMapState, scene::explorer::ecs::state::InitSpawnTileMapState};

mod setup;

pub struct ExplorerMapInitPlugin;

impl Plugin for ExplorerMapInitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(InitSpawnTileMapState::Running),
            ((spawn_startup_fullmap).run_if(in_state(FullMapState::On)),).run_if(run_once),
        )
        .add_systems(
            OnEnter(InitSpawnTileMapState::Running),
            ((spawn_startup_non_fullmap).run_if(in_state(FullMapState::Off)),).run_if(run_once),
        );
    }
}
