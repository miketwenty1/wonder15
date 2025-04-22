use animate::{animate_sprite, detect_fight, random_hal_walk};
use bevy::prelude::*;
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
            ));
    }
}
