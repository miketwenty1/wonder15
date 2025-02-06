use bevy::prelude::*;
use input::ExplorerInputPlugin;
use map::ExplorerMapPlugin;

use crate::helper::plugins::comms::CommsPlugin;

mod ecs;
mod input;
pub mod map;

pub struct ExplorerScenePlugin;

impl Plugin for ExplorerScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(SceneState::Explorer),
            (
                setup_animation
                // init_hardcoded_res,
                // setup_spritesheets,
                // init_js_comms_channels,
            )
                .chain()
                .run_if(run_once),
        )
        .add_sub_state::<ExplorerSubState>()
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
            CommsPlugin,
        ));
    }
}
