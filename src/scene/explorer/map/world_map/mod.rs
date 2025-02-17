use bevy::prelude::*;
use tile_world_hashmap::read_tile_update_event;

use crate::ecs::state::SceneState;

mod tile_world_hashmap;

pub struct ExplorerMapWorldMapPlugin;

impl Plugin for ExplorerMapWorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (read_tile_update_event,).run_if(in_state(SceneState::Explorer)),
        );
    }
}
