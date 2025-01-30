use bevy::math::UVec2;
use bevy_ecs_tilemap::map::{TilemapSpacing, TilemapTileSize};

pub const CHUNK_SIZE: UVec2 = UVec2 { x: 8, y: 8 };
pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};
pub const TILE_SPACING: TilemapSpacing = TilemapSpacing { x: 2.0, y: 2.0 };
pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 66.0, y: 66.0 };
