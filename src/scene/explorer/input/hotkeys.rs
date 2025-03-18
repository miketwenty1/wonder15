use bevy::prelude::*;

use crate::{
    helper::plugins::comms::ecs::event::GetBlockchainUpdates,
    scene::explorer::ecs::{
        event::{BuildingToggleEvent, SwapTilesEvent, TextToggleEvent},
        resource::ZoomLevelNumsRes,
    },
};

pub fn map_keyboard_hotkeys(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut swap_tile: EventWriter<SwapTilesEvent>,
    mut text_toggle: EventWriter<TextToggleEvent>,
    mut building_toggle: EventWriter<BuildingToggleEvent>,
    mut cam: Query<&mut Projection, With<Camera>>,
    // zoom_level_e: EventWriter<ZoomLevelEvent>,
    // zoom_res: ResMut<ZoomLevelRes>,
    zooms: Res<ZoomLevelNumsRes>,
    mut blockchain: EventWriter<GetBlockchainUpdates>,
) {
    let cam = cam.single_mut().unwrap().into_inner();
    let orthographic = match *cam {
        Projection::Orthographic(ref mut ortho) => ortho,
        _ => panic!("Expected Orthographic projection"),
    };

    if keyboard_input.just_pressed(KeyCode::Space) {
        swap_tile.write(SwapTilesEvent::Iter);
    }
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        text_toggle.write(TextToggleEvent::KeyPressToggle);
    }
    if keyboard_input.just_pressed(KeyCode::KeyB) {
        building_toggle.write(BuildingToggleEvent::KeyPressToggle);
    }
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        blockchain.write(GetBlockchainUpdates(0));
    }
    // digits 1-4
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        orthographic.scale = zooms.min_zoom;
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        orthographic.scale = zooms.close_threshold;
    } else if keyboard_input.just_pressed(KeyCode::Digit3) {
        orthographic.scale = zooms.medium_threshold;
    } else if keyboard_input.just_pressed(KeyCode::Digit4) {
        orthographic.scale = zooms.max_zoom;
    }
}
