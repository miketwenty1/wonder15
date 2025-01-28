use bevy::prelude::*;
use event::{ExplorerEventPlugin, SwapTilesEvent};
use input::ExplorerInputPlugin;
use map::ExplorerMapPlugin;
use resource::CurrentTilesRes;
use startup::{animate_sprite, setup_animation};

use super::{ExplorerSubState, SceneState};

mod component;
mod event;
mod hard;
mod input;
mod map;
mod resource;
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
        .insert_resource(CurrentTilesRes(SwapTilesEvent::PlayerColor))
        .add_plugins((ExplorerInputPlugin, ExplorerMapPlugin, ExplorerEventPlugin));
    }
}
