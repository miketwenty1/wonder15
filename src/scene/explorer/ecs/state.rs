use bevy::prelude::*;

use crate::ecs::state::SceneState;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(SceneState = SceneState::Explorer)]
pub enum ExplorerSubState {
    #[default]
    Off,
    Running,
    Paused,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(ExplorerSubState = ExplorerSubState::Running)]
pub enum ExplorerRunningZoomSub2State {
    #[default]
    Close,
    Medium,
    Far,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum InitSpawnTileMapState {
    #[default]
    Off,
    Running,
    Done,
}
