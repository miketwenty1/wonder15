use crate::ecs::{resource::WorldOwnedTileMap, state::SceneState};
use bevy::{prelude::*, utils::HashMap};
use camera::setup_camera;
use canvas::fit_canvas_to_parent;
use setup::{animate_sprite, setup_things};

pub mod camera;
pub mod canvas;
pub mod setup;

pub struct IniterInitPlugin;

impl Plugin for IniterInitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(SceneState::Init),
            (
                fit_canvas_to_parent,
                setup_camera,
                setup_things,
                // init_hardcoded_res,
                // setup_spritesheets,
                // init_js_comms_channels
            )
                .chain()
                .run_if(run_once),
        )
        .add_systems(Update, (animate_sprite).run_if(in_state(SceneState::Init)))
        .insert_resource(WorldOwnedTileMap {
            map: HashMap::new(),
        });
    }
}
