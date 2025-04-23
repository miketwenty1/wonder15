use bevy::prelude::*;

use crate::scene::explorer::ecs::{
    component::BlockchainFilterToggleParent,
    event::{BuildingToggleEvent, SpawnRunnerMan, SwapTilesEvent, TextToggleEvent},
    resource::ZoomLevelNumsRes,
};

#[allow(clippy::too_many_arguments)]
pub fn map_keyboard_hotkeys(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut swap_tile: EventWriter<SwapTilesEvent>,
    mut text_toggle: EventWriter<TextToggleEvent>,
    mut building_toggle: EventWriter<BuildingToggleEvent>,
    cam: Single<&mut Projection, With<Camera>>,
    // zoom_level_e: EventWriter<ZoomLevelEvent>,
    // zoom_res: ResMut<ZoomLevelRes>,
    zooms: Res<ZoomLevelNumsRes>,
    // mut blockchain: EventWriter<GetBlockchainUpdates>,
    mut blockchain_filter_parent_node_q: Query<&mut Node, With<BlockchainFilterToggleParent>>,
    mut spawn_man: EventWriter<SpawnRunnerMan>,
) {
    let mut binding = cam.into_inner();
    let ortho = if let Projection::Orthographic(ref mut ortho) = *binding {
        ortho
    } else {
        panic!("no ortho!");
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
        // blockchain.write(GetBlockchainUpdates(0));
        let mut di = blockchain_filter_parent_node_q.single_mut().unwrap();
        di.display = Display::Flex;
    }
    if keyboard_input.pressed(KeyCode::KeyO) {
        spawn_man.write(SpawnRunnerMan);
        spawn_man.write(SpawnRunnerMan);
        spawn_man.write(SpawnRunnerMan);
        spawn_man.write(SpawnRunnerMan);
        spawn_man.write(SpawnRunnerMan);
        spawn_man.write(SpawnRunnerMan);
        spawn_man.write(SpawnRunnerMan);
        spawn_man.write(SpawnRunnerMan);
        spawn_man.write(SpawnRunnerMan);
        spawn_man.write(SpawnRunnerMan);
        spawn_man.write(SpawnRunnerMan);
        spawn_man.write(SpawnRunnerMan);
        spawn_man.write(SpawnRunnerMan);
        spawn_man.write(SpawnRunnerMan);
        spawn_man.write(SpawnRunnerMan);
        spawn_man.write(SpawnRunnerMan);
        spawn_man.write(SpawnRunnerMan);
        spawn_man.write(SpawnRunnerMan);
        spawn_man.write(SpawnRunnerMan);
        spawn_man.write(SpawnRunnerMan);
    }
    // digits 1-4
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        ortho.scale = zooms.min_zoom;
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        ortho.scale = zooms.close_threshold;
    } else if keyboard_input.just_pressed(KeyCode::Digit3) {
        ortho.scale = zooms.medium_threshold;
    } else if keyboard_input.just_pressed(KeyCode::Digit4) {
        ortho.scale = zooms.max_zoom;
    }
}
