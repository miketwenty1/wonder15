use bevy::{asset::AssetMetaCheck, prelude::*};
use resource::{BlockchainHeight, GameHeight, GameStaticInputs, WinSize};
use scene::{
    explorer::ExplorerScenePlugin, init::InitScenePlugin, ExplorerRunningZoomSub2State,
    ExplorerSubState, SceneState,
};
use wasm_bindgen::prelude::wasm_bindgen;

mod helper;
mod resource;
mod scene;

pub fn main() {}

#[allow(clippy::too_many_arguments)]
#[wasm_bindgen]
pub fn game15(
    username: String,
    server_url: String,
    ln_address: String,
    curent_height: u32,
    browser_height: u32,
    blockchain_filters: bool,
    viewport_width: u32,
    viewport_height: u32,
    using_iphone: bool,
) {
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
        .insert_resource(GameStaticInputs {
            username,
            ln_address,
            using_iphone,
            server_url,
            blockchain_filters,
        })
        .insert_resource(GameHeight(browser_height))
        .insert_resource(WinSize {
            width: viewport_width as f32,
            height: viewport_height as f32,
        })
        .insert_resource(BlockchainHeight(curent_height))
        .init_state::<SceneState>()
        .add_sub_state::<ExplorerSubState>()
        .add_sub_state::<ExplorerRunningZoomSub2State>()
        .add_plugins((InitScenePlugin, ExplorerScenePlugin))
        .run();
}
