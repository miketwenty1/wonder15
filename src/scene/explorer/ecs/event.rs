use bevy::prelude::*;

use crate::resource::TileData;

pub struct ExplorerEventPlugin;

impl Plugin for ExplorerEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TextToggleEvent>()
            .add_event::<SwapTilesEvent>()
            .add_event::<BuildingToggleEvent>()
            .add_event::<UpdateTileTextureEvent>();
    }
}

#[derive(Event, Debug)]
pub enum TextToggleEvent {
    KeyPressToggle,
    ButtonToggle,
    Zoom,
}

#[derive(Event, Debug)]
pub enum BuildingToggleEvent {
    KeyPressToggle,
    ButtonToggle,
}

#[derive(Event, Debug, Clone, PartialEq)]
pub enum SwapTilesEvent {
    PlayerColor,
    Land,
    Iter,
}

impl SwapTilesEvent {
    /// Returns the next variant in the iteration sequence.
    pub fn next_tile_swap(&self) -> SwapTilesEvent {
        match self {
            SwapTilesEvent::PlayerColor => SwapTilesEvent::Land,
            SwapTilesEvent::Land => SwapTilesEvent::PlayerColor,
            // Define a default behavior for Iter or any unexpected variant
            _ => SwapTilesEvent::PlayerColor,
        }
    }
}

#[derive(Event, Debug)]
pub struct UpdateTileTextureEvent(pub Vec<TileData>);
