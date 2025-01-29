use bevy::prelude::*;

use crate::scene::explorer::event::{BuildingVisibilityEvent, SwapTilesEvent, TextVisibilityEvent};

pub fn map_keyboard_hotkeys(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut swap_tile: EventWriter<SwapTilesEvent>,
    mut text_visi: EventWriter<TextVisibilityEvent>,
    mut building_visi: EventWriter<BuildingVisibilityEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        swap_tile.send(SwapTilesEvent::Iter);
    }
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        text_visi.send(TextVisibilityEvent::KeyPressToggle);
    }
    if keyboard_input.just_pressed(KeyCode::KeyB) {
        building_visi.send(BuildingVisibilityEvent::KeyPressToggle);
    }
}
