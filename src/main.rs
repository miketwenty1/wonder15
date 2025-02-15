use bevy::{asset::AssetMetaCheck, prelude::*};
use ecs::{
    resource::{BlockchainHeight, GameHeight, GameStaticInputs, WinSize},
    state::{FullMapState, SceneState},
};
use scene::{explorer::ExplorerScenePlugin, initer::InitScenePlugin};

use wasm_bindgen::prelude::wasm_bindgen;

pub mod ecs;
mod helper;
pub mod scene;

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
    _screen_width: u32,
    _screen_height: u32,
    _device_pixel_ratio: f32,
    full_map_mode: bool,
    using_iphone: bool,
) {
    let window = Window {
        title: "SatoshiSettlers".to_string(),
        ..default()
    };
    let full_map_state = if full_map_mode {
        info!("full map mode on");
        FullMapState::On
    } else {
        FullMapState::Off
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
            full_map_mode,
        })
        .insert_resource(GameHeight(browser_height))
        .insert_resource(WinSize {
            width: viewport_width as f32,
            height: viewport_height as f32,
        })
        .insert_resource(BlockchainHeight(curent_height))
        .init_state::<SceneState>()
        .add_plugins((InitScenePlugin, ExplorerScenePlugin))
        .insert_state(full_map_state)
        .run();
}
