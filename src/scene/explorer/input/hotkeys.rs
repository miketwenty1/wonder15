use bevy::prelude::*;

use crate::scene::explorer::{
    event::{BuildingToggleEvent, SwapTilesEvent, TextToggleEvent},
    hard::{
        CLOSE_ZOOM_THRESHOLD, MAX_ZOOMIN_THRESHOLD, MAX_ZOOMOUT_THRESHOLD, MEDIUM_ZOOM_THRESHOLD,
    },
};

pub fn map_keyboard_hotkeys(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut swap_tile: EventWriter<SwapTilesEvent>,
    mut text_toggle: EventWriter<TextToggleEvent>,
    mut building_toggle: EventWriter<BuildingToggleEvent>,
    mut cam: Query<&mut OrthographicProjection, With<Camera>>,
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
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        for mut c in cam.iter_mut() {
            c.scale = MAX_ZOOMIN_THRESHOLD;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        for mut c in cam.iter_mut() {
            c.scale = CLOSE_ZOOM_THRESHOLD;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Digit3) {
        for mut c in cam.iter_mut() {
            c.scale = MEDIUM_ZOOM_THRESHOLD;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Digit4) {
        for mut c in cam.iter_mut() {
            c.scale = MAX_ZOOMOUT_THRESHOLD;
        }
    }
}
