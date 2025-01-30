use super::event::{SwapTilesEvent, ZoomLevelEvent};
use bevy::prelude::Resource;

#[derive(Resource, Debug, Clone)]
pub struct CurrentTilesRes(pub SwapTilesEvent);

#[derive(Resource, Debug, PartialEq)]
pub struct ZoomLevelRes(pub ZoomLevelEvent);
