use std::collections::HashMap;

use crate::ecs::{resource::WorldOwnedTileMap, state::SceneState};
use bevy::prelude::*;
use startup::{init_startup, spawn_running_hal};

pub mod startup;

pub struct ExplorerInitPlugin;

impl Plugin for ExplorerInitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(SceneState::Explorer),
            (init_startup).run_if(run_once),
        )
        .add_systems(Update, spawn_running_hal)
        .insert_resource(WorldOwnedTileMap {
            map: HashMap::new(),
        });
    }
}
