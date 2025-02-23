use bevy::prelude::*;
use blockchain_world_hashmap::read_blockchain_world_update_event;
use tile_world_hashmap::read_game_world_update_event;

use crate::ecs::state::SceneState;

mod blockchain_world_hashmap;
mod tile_world_hashmap;

pub struct ExplorerMapWorldMapPlugin;

impl Plugin for ExplorerMapWorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                read_blockchain_world_update_event,
                read_game_world_update_event,
            )
                .run_if(in_state(SceneState::Explorer)),
        );
    }
}
