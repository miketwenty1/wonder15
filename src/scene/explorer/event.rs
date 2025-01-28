use bevy::prelude::*;

pub struct ExplorerEventPlugin;

impl Plugin for ExplorerEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TextVisibilityEvent>()
            .add_event::<SwapTilesEvent>();
    }
}

#[derive(Event, Debug)]
pub enum TextVisibilityEvent {
    KeyPressToggle,
    ButtonToggle,
    Zoom,
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
