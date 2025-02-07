use bevy::math::UVec2;
use bevy_ecs_tilemap::map::{TilemapSpacing, TilemapTileSize};

pub const TEXT_CHUNK_SIZE: UVec2 = UVec2 { x: 8, y: 8 };
pub const TEXT_RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: TEXT_CHUNK_SIZE.x * 2,
    y: TEXT_CHUNK_SIZE.y * 2,
};
pub const BUILDING_CHUNK_SIZE: UVec2 = UVec2 { x: 12, y: 12 };
pub const BUILDING_RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: BUILDING_CHUNK_SIZE.x * 2,
    y: BUILDING_CHUNK_SIZE.y * 2,
};
pub const TILE_CHUNK_SIZE: UVec2 = UVec2 { x: 12, y: 12 };
pub const TILE_RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: TILE_CHUNK_SIZE.x * 2,
    y: TILE_CHUNK_SIZE.y * 2,
};
pub const TILE_SPACING: TilemapSpacing = TilemapSpacing { x: 2.0, y: 2.0 };
pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 66.0, y: 66.0 };
pub const TILE_DESPAWN_RANGE: f32 = TILE_CHUNK_SIZE.x as f32 * TILE_SIZE.x * 8.0;
pub const BUILDING_DESPAWN_RANGE: f32 = BUILDING_CHUNK_SIZE.x as f32 * TILE_SIZE.x * 8.0;
pub const TEXT_DESPAWN_RANGE: f32 = TEXT_CHUNK_SIZE.x as f32 * TILE_SIZE.x * 4.0;

pub const CHUNK_INIT_LOAD_SIZE: u32 = 10_000;
