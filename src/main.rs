use bevy::{asset::AssetMetaCheck, prelude::*};
use scene::{
    explorer::{map::setup::startup, GameExplorerPlugin},
    init::GameInitPlugin,
    ExplorerSubState, SceneState,
};
use wasm_bindgen::prelude::wasm_bindgen;

mod helper;
mod scene;

pub fn main() {}

#[wasm_bindgen]
pub fn game15() {
    let window = Window {
        title: "SatoshiSettlers".to_string(),
        ..default()
    };
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(window),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .init_state::<SceneState>()
        .add_sub_state::<ExplorerSubState>()
        .add_plugins((GameInitPlugin, GameExplorerPlugin))
        .add_systems(Startup, startup)
        .run();
}
