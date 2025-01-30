use bevy::{asset::AssetMetaCheck, prelude::*};
use scene::{
    explorer::ExplorerScenePlugin, init::InitScenePlugin, ExplorerRunningZoomSub2State,
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
        .add_sub_state::<ExplorerRunningZoomSub2State>()
        .add_plugins((InitScenePlugin, ExplorerScenePlugin))
        .run();
}
