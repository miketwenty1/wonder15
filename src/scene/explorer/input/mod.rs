use bevy::prelude::*;
use hard::{
    AddTileManualSelectionSprite, CursorPosInfo, CursorPosRaw, LastClickedTile,
    RemoveTileManualSelectionSprite,
};
use hotkeys::map_keyboard_hotkeys;
use mouse_movement::mouse_movement_camera_system;
use mouse_tile_tracking::{attribute_click_on_map, cursor_to_tile, update_cursor_pos};
use movement::keyboard_movement;
use zoom_helper::changed_ortho;
use zoom_keys::zoom_keyboard;
use zoom_mouse_wheel::zoom_wheel_system;

use super::ecs::state::ExplorerSubState;

pub mod hard;
mod hotkeys;
mod mouse_movement;
mod mouse_tile_tracking;
mod movement;
mod zoom_helper;
mod zoom_keys;
mod zoom_mouse_wheel;

pub struct ExplorerInputPlugin;

impl Plugin for ExplorerInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosRaw>()
            .init_resource::<CursorPosInfo>()
            .init_resource::<LastClickedTile>()
            .add_event::<AddTileManualSelectionSprite>()
            .add_event::<RemoveTileManualSelectionSprite>()
            .add_systems(
                Update,
                (
                    keyboard_movement,
                    zoom_keyboard,
                    map_keyboard_hotkeys,
                    zoom_wheel_system,
                    changed_ortho,
                    mouse_movement_camera_system,
                    update_cursor_pos,
                    cursor_to_tile,
                    attribute_click_on_map,
                )
                    .run_if(in_state(ExplorerSubState::Running)),
            );
    }
}
