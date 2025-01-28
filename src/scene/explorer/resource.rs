use super::event::SwapTilesEvent;
use bevy::prelude::Resource;

#[derive(Resource, Debug, Clone)]
pub struct CurrentTilesRes(pub SwapTilesEvent);
