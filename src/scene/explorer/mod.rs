use animate::animate_sprite;
use bevy::prelude::*;
use ecs::{
    event::{ExplorerEventPlugin, SwapTilesEvent},
    resource::{CurrentTilesRes, ZoomLevelRes},
    state::{ExplorerRunningZoomSub2State, ExplorerSubState},
};
use init::ExplorerInitPlugin;
use input::ExplorerInputPlugin;
use map::ExplorerMapPlugin;

use crate::{ecs::state::SceneState, helper::plugins::comms::CommsPlugin};

mod animate;
mod ecs;
mod init;
mod input;
mod map;
mod ui;

pub struct ExplorerScenePlugin;

impl Plugin for ExplorerScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<ExplorerSubState>()
            .add_sub_state::<ExplorerRunningZoomSub2State>()
            .add_systems(
                Update,
                (animate_sprite).run_if(in_state(ExplorerSubState::Running)),
            )
            .insert_resource(CurrentTilesRes(SwapTilesEvent::PlayerColor))
            .insert_resource(ZoomLevelRes(ExplorerRunningZoomSub2State::Close))
            .add_plugins((
                ExplorerInputPlugin,
                ExplorerMapPlugin,
                ExplorerEventPlugin,
                ExplorerInitPlugin,
            ));
    }
}
