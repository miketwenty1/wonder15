use std::collections::HashMap;

use crate::ecs::{resource::WorldOwnedTileMap, state::SceneState};
use bevy::prelude::*;
use startup::{init_startup, setup_animation};

pub mod startup;

pub struct ExplorerInitPlugin;

impl Plugin for ExplorerInitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(SceneState::Explorer),
            (init_startup, setup_animation).chain().run_if(run_once),
        )
        .insert_resource(WorldOwnedTileMap {
            map: HashMap::new(),
        });
    }
}
