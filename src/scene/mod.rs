use bevy::prelude::*;

pub mod explorer;
pub mod init;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum SceneState {
    #[default]
    Init,
    Explorer,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(SceneState = SceneState::Explorer)]
pub enum ExplorerSubState {
    #[default]
    Running,
    Paused,
}
