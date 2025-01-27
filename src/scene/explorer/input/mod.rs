mod movement;
mod zoom_mouse_wheel;

use bevy::prelude::*;
use movement::keyboard_movement;
use zoom_mouse_wheel::zoom_wheel_system;

use crate::scene::ExplorerSubState;

pub struct ExplorerInputPlugin;

impl Plugin for ExplorerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (keyboard_movement, zoom_wheel_system).run_if(in_state(ExplorerSubState::Running)),
        );
    }
}
