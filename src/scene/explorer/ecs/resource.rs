use super::event::SwapTilesEvent;
use crate::scene::explorer::ecs::state::ExplorerRunningZoomSub2State;
use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct CurrentTilesRes(pub SwapTilesEvent);

#[derive(Resource, Debug, PartialEq)]
pub struct ZoomLevelRes(pub ExplorerRunningZoomSub2State);

#[derive(Resource, Clone)]
pub struct SpriteSheetBuildingRes {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}
