use bevy::prelude::*;

use crate::scene::explorer::event::{BuildingToggleEvent, SwapTilesEvent, TextToggleEvent};

pub fn map_keyboard_hotkeys(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut swap_tile: EventWriter<SwapTilesEvent>,
    mut text_toggle: EventWriter<TextToggleEvent>,
    mut building_toggle: EventWriter<BuildingToggleEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        swap_tile.send(SwapTilesEvent::Iter);
    }
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        text_toggle.send(TextToggleEvent::KeyPressToggle);
    }
    if keyboard_input.just_pressed(KeyCode::KeyB) {
        building_toggle.send(BuildingToggleEvent::KeyPressToggle);
    }
}
