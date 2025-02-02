use bevy::prelude::*;

use crate::scene::explorer::{
    event::{BuildingToggleEvent, SwapTilesEvent, TextToggleEvent, ZoomLevelEvent},
    hard::{
        CLOSE_ZOOM_THRESHOLD, MAX_ZOOMIN_THRESHOLD, MAX_ZOOMOUT_THRESHOLD, MEDIUM_ZOOM_THRESHOLD,
    },
    resource::ZoomLevelRes,
};

use super::zoom_helper::zoom_event_writer;

pub fn map_keyboard_hotkeys(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut swap_tile: EventWriter<SwapTilesEvent>,
    mut text_toggle: EventWriter<TextToggleEvent>,
    mut building_toggle: EventWriter<BuildingToggleEvent>,
    mut cam: Query<&mut OrthographicProjection, With<Camera>>,
    zoom_level_e: EventWriter<ZoomLevelEvent>,
    zoom_res: ResMut<ZoomLevelRes>,
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
    // digits 1-4
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        let mut c = cam.get_single_mut().unwrap();
        c.scale = MAX_ZOOMIN_THRESHOLD;
        zoom_event_writer(MAX_ZOOMIN_THRESHOLD, zoom_level_e, zoom_res);
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        let mut c = cam.get_single_mut().unwrap();
        c.scale = CLOSE_ZOOM_THRESHOLD;
        zoom_event_writer(CLOSE_ZOOM_THRESHOLD, zoom_level_e, zoom_res);
    } else if keyboard_input.just_pressed(KeyCode::Digit3) {
        let mut c = cam.get_single_mut().unwrap();
        c.scale = MEDIUM_ZOOM_THRESHOLD;
        zoom_event_writer(MEDIUM_ZOOM_THRESHOLD, zoom_level_e, zoom_res);
    } else if keyboard_input.just_pressed(KeyCode::Digit4) {
        let mut c = cam.get_single_mut().unwrap();
        c.scale = MAX_ZOOMOUT_THRESHOLD;
        zoom_event_writer(MAX_ZOOMOUT_THRESHOLD, zoom_level_e, zoom_res);
    }
}
