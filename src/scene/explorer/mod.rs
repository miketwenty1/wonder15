use crate::helper::plugins::comms::ecs::resource::CommsChannel;
use animate::{animate_sprite, detect_fight, random_hal_walk};
use bevy::{prelude::*, tasks::IoTaskPool};
use ecs::{
    event::{ExplorerEventPlugin, SwapTilesEvent},
    resource::{CurrentTilesRes, CurrentZoomLevelRes, MouseSelectedTile},
    state::{ExplorerRunningZoomSub2State, ExplorerSubState, InitSpawnMapState},
};
use general_button_behavior::general_btn;
use init::ExplorerInitPlugin;
use input::ExplorerInputPlugin;
use map::ExplorerMapPlugin;
use tile_cart::TileCartPlugin;
use ui::ExplorerUiPlugin;
use wasm_bindgen_futures::spawn_local;
mod animate;
//mod blockchain_color;
mod cart_details_menu;
pub mod ecs;
pub mod general_button_behavior;
mod init;
mod input;
mod map;
pub mod tile_cart;
pub mod ui;

pub struct ExplorerScenePlugin;

impl Plugin for ExplorerScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<ExplorerSubState>()
            .add_sub_state::<ExplorerRunningZoomSub2State>()
            .add_systems(
                Update,
                (animate_sprite, random_hal_walk, detect_fight).run_if(
                    in_state(ExplorerSubState::Running).and(in_state(InitSpawnMapState::Done)),
                ),
            )
            .add_systems(Update, general_btn)
            .insert_resource(CurrentTilesRes(SwapTilesEvent::PlayerColor))
            .insert_resource(CurrentZoomLevelRes(ExplorerRunningZoomSub2State::Close))
            .insert_resource(MouseSelectedTile(999_999_999))
            .add_plugins((
                ExplorerInputPlugin,
                ExplorerMapPlugin,
                ExplorerEventPlugin,
                ExplorerInitPlugin,
                ExplorerUiPlugin,
                TileCartPlugin,
            ))
            .insert_resource(ApiRequestTimer(Timer::from_seconds(
                5.0,
                TimerMode::Repeating,
            )))
            .add_systems(Update, test_send_api_req);
    }
}

#[derive(Resource, Default)]
pub struct ApiRequestTimer(Timer);

// System to trigger API calls to all endpoints
// System to trigger API calls every 10 seconds
pub fn test_send_api_req(
    mut channel: ResMut<CommsChannel>,
    time: Res<Time>,
    mut timer: ResMut<ApiRequestTimer>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }

    let endpoints = vec![
        "browser_blockchain_data",
        "browser_game_tiles",
        "server_blockchain_data",
        "game_tiles_by_ts",
        "game_tiles_by_height",
        "check_ln_payment",
        "ln_invoice",
        "user_inventory",
        "block_data",
    ];

    for endpoint in endpoints {
        let pool = IoTaskPool::get();
        let cc = channel.tx.clone();
        let _task = pool.spawn(async move {
            let api_response_r =
                reqwest::get(format!("http://localhost:8081/comms/{}", endpoint)).await;
            match api_response_r {
                Ok(o) => {
                    let api_response_text_r = o.text().await;
                    match api_response_text_r {
                        Ok(o) => {
                            let _ = cc.try_send(o);
                        }
                        Err(e) => {
                            info!("failed to parse to message text: {:#?}", e);
                            let _ = cc.try_send(e.to_string());
                        }
                    }
                }
                Err(e) => {
                    info!("failed to send in channel message: {:#?}", e);
                    let _ = cc.try_send(e.to_string());
                }
            }
        });
    }
}
