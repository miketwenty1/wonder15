use bevy::math::UVec2;
use bevy_ecs_tilemap::map::{TilemapSpacing, TilemapTileSize};

pub const CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };
pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};
pub const TILE_SPACING: TilemapSpacing = TilemapSpacing { x: 2.0, y: 2.0 };
pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 66.0, y: 66.0 };
pub const BUILDING_DESPAWN_RANGE: f32 = CHUNK_SIZE.x as f32 * TILE_SIZE.x * 16.0;
pub const TEXT_DESPAWN_RANGE: f32 = CHUNK_SIZE.x as f32 * TILE_SIZE.x * 2.0;
