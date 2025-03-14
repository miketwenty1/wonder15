use super::event::SwapTilesEvent;
use crate::scene::explorer::ecs::state::ExplorerRunningZoomSub2State;
use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct CurrentTilesRes(pub SwapTilesEvent);

#[derive(Resource, Debug, PartialEq)]
pub struct CurrentZoomLevelRes(pub ExplorerRunningZoomSub2State);

#[derive(Resource, Clone)]
pub struct SpriteSheetBuildingRes {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

#[derive(Resource, Clone)]
pub struct ZoomLevelNumsRes {
    pub max_zoom: f32,
    pub min_zoom: f32,
    pub close_threshold: f32,
    pub medium_threshold: f32,
}

#[derive(Resource, Clone)]
pub struct ChunkTypeNumsRes {
    pub building: UVec3, // x, y, and value
    pub tile: UVec2,
    pub text: UVec2,
}

#[derive(Resource, Debug)]
pub struct DespawnTileRangeRes(pub f32);

#[derive(Resource, Debug)]
pub struct DespawnTextRangeRes(pub f32);

#[derive(Resource, Debug)]
pub struct DespawnBuildingRangeRes(pub f32);
