use bevy::{color::Srgba, math::UVec2};
use bevy_ecs_tilemap::map::TilemapTileSize;

pub const TILE_Z: f32 = 2.0;
pub const BUILDING_Z: f32 = 3.0;
pub const TEXT_Z: f32 = 8.0;
pub const ANIMATED_SPRITE_Z: f32 = 30.5;

pub const MAX_ZOOM: f32 = 94.0;
pub const MIN_ZOOM: f32 = 0.40;
pub const CLOSE_ZOOM_THRESHOLD: f32 = 2.5;
pub const MEDIUM_ZOOM_THRESHOLD: f32 = 5.0;

pub const SLIM_MAX_ZOOM: f32 = 5.0;
pub const SLIM_CLOSE_ZOOM_THRESHOLD: f32 = 1.0;
pub const SLIM_MEDIUM_ZOOM_THRESHOLD: f32 = 2.0;

pub const TEXT_CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };
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

pub const TILE_SPAN_SPAWN_NUMBER: i32 = 2;
pub const BUILDING_SPAN_SPAWN_NUMBER: i32 = 4;
pub const TEXT_SPAN_SPAWN_NUMBER: i32 = 2;

pub const TILE_DESPAWN_RANGE_MULTIPLIER: f32 = (2 * TILE_SPAN_SPAWN_NUMBER) as f32;
pub const BUILDING_DESPAWN_RANGE_MULTIPLIER: f32 = (2 * BUILDING_SPAN_SPAWN_NUMBER) as f32;
pub const TEXT_DESPAWN_RANGE_MULTIPLIER: f32 = (2 * TEXT_SPAN_SPAWN_NUMBER) as f32;
pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 98.0, y: 98.0 };

pub const DARKEST_ALLOWED_BUILDING: Srgba = Srgba {
    red: 0.2,
    green: 0.2,
    blue: 0.2,
    alpha: 1.0,
};

pub const UI_SMALL_TEXT_SIZE: f32 = 15.0;
pub const UI_MEDIUM_TEXT_SIZE: f32 = 20.0;
pub const UI_LARGE_TEXT_SIZE: f32 = 30.0;
pub const UI_LARGE_BUTTON_WIDTH: f32 = 75.0;
pub const UI_LARGE_BUTTON_HEIGHT: f32 = 45.0;

pub const UI_ICON_SIZE: f32 = 60.0;

pub const TILE_MAP_LENGTH: u32 = 1000;

pub const MINIMUM_COST_AMOUNT: u32 = 32;
