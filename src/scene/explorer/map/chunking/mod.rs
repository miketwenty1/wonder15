use bevy::prelude::*;
use chunk_full_map::startup_fullmap;
use chunking_building::{despawn_building_outofrange_chunks, spawn_building_chunks_around_camera};
use chunking_text2d::{despawn_text_outofrange_chunks, spawn_text_chunk_around_camera};
use chunking_tiles::{despawn_tile_outofrange_chunks, spawn_tile_chunks_around_camera};
use get_tile_trigger::init_local_storage_read;

use super::ecs::state::{BuildingToggleState, TextToggleState};
use crate::{
    ecs::state::FullMapState,
    scene::explorer::ecs::state::{
        ExplorerRunningZoomSub2State, ExplorerSubState, InitSpawnMapState,
    },
};

mod building_config;
mod chunk_full_map;
mod chunking_building;
mod chunking_text2d;
mod chunking_tiles;
mod get_tile_trigger;

pub struct ExplorerMapChunkingPlugin;

impl Plugin for ExplorerMapChunkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (startup_fullmap)
                .run_if(in_state(InitSpawnMapState::MapSpawn).and(in_state(FullMapState::On))),
        )
        .add_systems(
            Update,
            (init_local_storage_read)
                .run_if(in_state(InitSpawnMapState::LocalStorageRead).and(run_once)),
        )
        .add_systems(
            Update,
            (
                despawn_text_outofrange_chunks,
                spawn_text_chunk_around_camera,
            )
                .run_if(
                    in_state(ExplorerRunningZoomSub2State::Close)
                        .and(in_state(TextToggleState::On)),
                ),
        )
        .add_systems(
            Update,
            (
                despawn_building_outofrange_chunks,
                spawn_building_chunks_around_camera,
            )
                .run_if(
                    in_state(ExplorerSubState::Running).and(
                        in_state(BuildingToggleState::On)
                            .and(not(in_state(ExplorerRunningZoomSub2State::Far))),
                    ),
                ),
        )
        .add_systems(
            Update,
            (
                despawn_tile_outofrange_chunks,
                spawn_tile_chunks_around_camera,
            )
                .run_if(in_state(FullMapState::Off).and(in_state(ExplorerSubState::Running))),
        );
    }
}
