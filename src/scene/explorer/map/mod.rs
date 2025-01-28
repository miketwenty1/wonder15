use bevy::prelude::*;
use bevy_ecs_tilemap::map::{TilemapSpacing, TilemapTileSize};
use resource::{
    AdditionalSetupTilesTimerRes, ChunkManagerRes, DespawnRangeRes, TotalTilesSpawnedRes,
};
use setup::{startup, startup_tilemap};
use state::InitSpawnTileMapState;
use swap_tiles::swap_tile_index_reader;
use text2d_chunking::{despawn_outofrange_chunks, spawn_chunks_around_camera};
use text_visibility::text_visibility_reader;

mod component;
mod resource;
mod setup;
mod state;
mod swap_tiles;
mod text2d_chunking;
mod text_visibility;

use crate::scene::ExplorerSubState;

pub const CHUNK_SIZE: UVec2 = UVec2 { x: 4, y: 4 };
pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};
pub const TILE_SPACING: TilemapSpacing = TilemapSpacing { x: 2.0, y: 2.0 };
const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 66.0, y: 66.0 };

pub struct ExplorerMapPlugin;

impl Plugin for ExplorerMapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AdditionalSetupTilesTimerRes(Timer::from_seconds(
            0.05,
            TimerMode::Repeating,
        )))
        .insert_resource(ChunkManagerRes::default())
        .insert_resource(TotalTilesSpawnedRes(0))
        .insert_resource(DespawnRangeRes(CHUNK_SIZE.x as f32 * TILE_SIZE.x * 6.0))
        .init_state::<InitSpawnTileMapState>()
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
                despawn_outofrange_chunks,
                spawn_chunks_around_camera,
                swap_tile_index_reader,
                text_visibility_reader,
            )
                .run_if(in_state(ExplorerSubState::Running)),
        );
    }
}
