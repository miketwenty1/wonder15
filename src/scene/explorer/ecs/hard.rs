use bevy::math::UVec2;
use bevy_ecs_tilemap::map::TilemapTileSize;

pub const TILE_Z: f32 = 2.0;
pub const BUILDING_Z: f32 = 3.0;
pub const TEXT_Z: f32 = 4.0;
pub const ANIMATED_SPRITE_Z: f32 = 4.5;

pub const MAX_ZOOM: f32 = 94.0;
pub const MIN_ZOOM: f32 = 0.40;
pub const CLOSE_ZOOM_THRESHOLD: f32 = 1.25;
pub const MEDIUM_ZOOM_THRESHOLD: f32 = 3.0;

pub const SLIM_MAX_ZOOM: f32 = 5.0;
pub const SLIM_CLOSE_ZOOM_THRESHOLD: f32 = 1.0;
pub const SLIM_MEDIUM_ZOOM_THRESHOLD: f32 = 2.0;

pub const TEXT_CHUNK_SIZE: UVec2 = UVec2 { x: 12, y: 8 };
pub const TEXT_RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: TEXT_CHUNK_SIZE.x * 2,
    y: TEXT_CHUNK_SIZE.y * 2,
};
pub const BUILDING_CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };
pub const BUILDING_RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: BUILDING_CHUNK_SIZE.x * 2,
    y: BUILDING_CHUNK_SIZE.y * 2,
};
pub const TILE_CHUNK_SIZE: UVec2 = UVec2 { x: 40, y: 40 };
pub const TILE_RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: TILE_CHUNK_SIZE.x * 2,
    y: TILE_CHUNK_SIZE.y * 2,
};
pub const SLIM_TEXT_CHUNK_SIZE: UVec2 = UVec2 { x: 8, y: 8 };
pub const SLIM_TEXT_RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: SLIM_TEXT_CHUNK_SIZE.x * 2,
    y: SLIM_TEXT_CHUNK_SIZE.y * 2,
};
pub const SLIM_BUILDING_CHUNK_SIZE: UVec2 = UVec2 { x: 12, y: 12 };
pub const SLIM_BUILDING_RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: SLIM_BUILDING_CHUNK_SIZE.x * 2,
    y: SLIM_BUILDING_CHUNK_SIZE.y * 2,
};
pub const SLIM_TILE_CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };
pub const SLIM_TILE_RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: SLIM_TILE_CHUNK_SIZE.x * 2,
    y: SLIM_TILE_CHUNK_SIZE.y * 2,
};
pub const TILE_DESPAWN_RANGE_MULTIPLIER: f32 = 4.0;
pub const BUILDING_DESPAWN_RANGE_MULTIPLIER: f32 = 4.0;
pub const TEXT_DESPAWN_RANGE_MULTIPLIER: f32 = 4.0;
pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 98.0, y: 98.0 };
