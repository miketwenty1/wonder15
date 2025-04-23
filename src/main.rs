use bevy::{asset::AssetMetaCheck, prelude::*};
use ecs::{
    resource::{
        BlockchainFiltersHeight, BlockchainHeight, GameHeight, GameStaticInputs, GameTimetamp,
        WinSize,
    },
    state::{FullMapState, SceneState},
};
use helper::utils::funs::str_to_dateutc;
use scene::initer::InitScenePlugin;

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
    curent_blockchain_height: u32,
    game_ts: String,
    blockchain_filters_height: u32,
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
    let gts = if game_ts.is_empty() {
        None
    } else {
        Some(str_to_dateutc(game_ts))
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
            full_map_mode,
        })
        .insert_resource(GameHeight(0))
        .insert_resource(BlockchainFiltersHeight(blockchain_filters_height))
        .insert_resource(WinSize {
            width: viewport_width as f32,
            height: viewport_height as f32,
        })
        .insert_resource(BlockchainHeight(curent_blockchain_height))
        .init_state::<SceneState>()
        .add_plugins(InitScenePlugin)
        .insert_state(full_map_state)
        .insert_resource(GameTimetamp { ts: gts })
        .run();
}
