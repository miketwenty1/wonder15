use crate::scene::ExplorerRunningZoomSub2State;

use super::event::SwapTilesEvent;
use bevy::prelude::Resource;

#[derive(Resource, Debug, Clone)]
pub struct CurrentTilesRes(pub SwapTilesEvent);

#[derive(Resource, Debug, PartialEq)]
pub struct ZoomLevelRes(pub ExplorerRunningZoomSub2State);
