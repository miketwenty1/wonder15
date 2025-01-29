use bevy::prelude::*;
use bevy_ecs_tilemap::map::{TilemapSpacing, TilemapTileSize};
use chunking_building::{
    despawn_building_outofrange_chunks, despawn_building_outofzoom_chunks,
    spawn_building_chunks_around_camera,
};
use chunking_text2d::{
    despawn_text_outofrange_chunks, despawn_text_outofzoom_chunks, spawn_text_chunks_around_camera,
};
use resource::MapResPlugin;
use setup::{startup, startup_tilemap};
use state::{BuildingVisibilityState, InitSpawnTileMapState, TextVisibilityState};
use swap_tiles::swap_tile_index_reader;
use visibility_building::building_visibility_reader;
use visibility_text::text_visibility_reader;

mod chunking_building;
mod chunking_text2d;
mod component;
mod resource;
mod setup;
mod state;
mod swap_tiles;
mod visibility_building;
mod visibility_text;

use crate::scene::{ExplorerRunningSub2State, ExplorerSubState};

pub const CHUNK_SIZE: UVec2 = UVec2 { x: 8, y: 8 };
pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};
pub const TILE_SPACING: TilemapSpacing = TilemapSpacing { x: 2.0, y: 2.0 };
const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 66.0, y: 66.0 };
const BUILDING_SPRITE_SIZE: TilemapTileSize = TilemapTileSize { x: 34.0, y: 34.0 };

pub struct ExplorerMapPlugin;

impl Plugin for ExplorerMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MapResPlugin)
            .init_state::<InitSpawnTileMapState>()
            .init_state::<BuildingVisibilityState>()
            .init_state::<TextVisibilityState>()
            .add_systems(
                OnEnter(InitSpawnTileMapState::Running),
                (startup).run_if(run_once),
            )
            .add_systems(
                Update,
                (startup_tilemap).run_if(in_state(InitSpawnTileMapState::Running)),
            )
            .add_systems(
                Update,
                (
                    swap_tile_index_reader,
                    building_visibility_reader,
                    text_visibility_reader,
                )
                    .run_if(in_state(ExplorerSubState::Running)),
            )
            .add_systems(
                Update,
                (
                    despawn_text_outofrange_chunks,
                    spawn_text_chunks_around_camera,
                )
                    .run_if(
                        in_state(ExplorerRunningSub2State::ZoomClose)
                            .and(in_state(TextVisibilityState::On)),
                    ),
            )
            .add_systems(
                OnExit(ExplorerRunningSub2State::ZoomClose),
                despawn_text_outofzoom_chunks,
            )
            .add_systems(
                Update,
                (
                    despawn_building_outofrange_chunks,
                    spawn_building_chunks_around_camera,
                )
                    .run_if(
                        not(in_state(ExplorerRunningSub2State::ZoomFar))
                            .and(in_state(BuildingVisibilityState::On)),
                    ),
            )
            .add_systems(
                OnEnter(ExplorerRunningSub2State::ZoomFar),
                despawn_building_outofzoom_chunks,
            );
    }
}
