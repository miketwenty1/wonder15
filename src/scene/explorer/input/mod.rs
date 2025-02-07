use bevy::prelude::*;
use hotkeys::map_keyboard_hotkeys;
use mouse_movement::mouse_movement_camera_system;
use movement::keyboard_movement;
use zoom_helper::changed_ortho;
use zoom_keys::zoom_keyboard;
use zoom_mouse_wheel::zoom_wheel_system;

use super::ecs::state::ExplorerSubState;

mod hard;
mod hotkeys;
mod mouse_movement;
mod movement;
mod zoom_helper;
mod zoom_keys;
mod zoom_mouse_wheel;

pub struct ExplorerInputPlugin;

impl Plugin for ExplorerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                keyboard_movement,
                zoom_keyboard,
                map_keyboard_hotkeys,
                zoom_wheel_system,
                changed_ortho,
                mouse_movement_camera_system,
            )
                .run_if(in_state(ExplorerSubState::Running)),
        );
    }
}
