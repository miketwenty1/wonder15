use bevy::prelude::*;
use input::ExplorerInputPlugin;
use map::ExplorerMapPlugin;
use startup::{animate_sprite, setup_animation};

use super::{ExplorerSubState, SceneState};

mod component;
mod input;
mod map;
mod startup;

pub struct ExplorerScenePlugin;

impl Plugin for ExplorerScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(SceneState::Explorer),
            (
                setup_animation
                // init_hardcoded_res,
                // setup_spritesheets,
                // init_js_comms_channels,
            )
                .chain()
                .run_if(run_once),
        )
        .add_systems(
            Update,
            (animate_sprite).run_if(in_state(ExplorerSubState::Running)),
        )
        .add_plugins((ExplorerInputPlugin, ExplorerMapPlugin));
    }
}
