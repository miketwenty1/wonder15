use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use camera::setup_camera;
use canvas::fit_canvas_to_parent;
use startup::{animate_sprite, setup_animation};

use super::SceneState;

mod camera;
mod canvas;
mod component;
mod resource;
mod startup;

pub struct InitScenePlugin;

impl Plugin for InitScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .add_systems(
                OnEnter(SceneState::Init),
                (
                    fit_canvas_to_parent,
                    setup_camera,
                    setup_animation,
                    // init_hardcoded_res,
                    // setup_spritesheets,
                    // init_js_comms_channels
                )
                    .chain()
                    .run_if(run_once),
            )
            .enable_state_scoped_entities::<SceneState>()
            .add_systems(Update, (animate_sprite).run_if(in_state(SceneState::Init)));
    }
}
