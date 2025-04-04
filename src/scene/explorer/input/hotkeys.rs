use bevy::prelude::*;

use crate::{
    helper::plugins::comms::ecs::event::GetBlockchainUpdates,
    scene::explorer::ecs::{
        component::BlockchainFilterToggleParent,
        event::{BuildingToggleEvent, SwapTilesEvent, TextToggleEvent},
        resource::ZoomLevelNumsRes,
    },
};

#[allow(clippy::too_many_arguments)]
pub fn map_keyboard_hotkeys(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut swap_tile: EventWriter<SwapTilesEvent>,
    mut text_toggle: EventWriter<TextToggleEvent>,
    mut building_toggle: EventWriter<BuildingToggleEvent>,
    mut cam: Query<&mut OrthographicProjection, With<Camera>>,
    // zoom_level_e: EventWriter<ZoomLevelEvent>,
    // zoom_res: ResMut<ZoomLevelRes>,
    zooms: Res<ZoomLevelNumsRes>,
    mut blockchain: EventWriter<GetBlockchainUpdates>,
    mut blockchain_filter_parent_node_q: Query<&mut Node, With<BlockchainFilterToggleParent>>,
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
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        blockchain.send(GetBlockchainUpdates(0));
        let mut di = blockchain_filter_parent_node_q.get_single_mut().unwrap();
        di.display = Display::Flex;
    }
    // digits 1-4
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        let mut c = cam.get_single_mut().unwrap();
        c.scale = zooms.min_zoom;
        // zoom_event_writer(MAX_ZOOMIN_THRESHOLD, zoom_level_e, zoom_res);
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        let mut c = cam.get_single_mut().unwrap();
        c.scale = zooms.close_threshold;
        // zoom_event_writer(CLOSE_ZOOM_THRESHOLD, zoom_level_e, zoom_res);
    } else if keyboard_input.just_pressed(KeyCode::Digit3) {
        let mut c = cam.get_single_mut().unwrap();
        c.scale = zooms.medium_threshold;
        //  zoom_event_writer(MEDIUM_ZOOM_THRESHOLD, zoom_level_e, zoom_res);
    } else if keyboard_input.just_pressed(KeyCode::Digit4) {
        let mut c = cam.get_single_mut().unwrap();
        c.scale = zooms.max_zoom;
        // zoom_event_writer(MAX_ZOOMOUT_THRESHOLD, zoom_level_e, zoom_res);
    }
}
