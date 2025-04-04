use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum FullMapState {
    On,
    #[default]
    Off,
}
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum SceneState {
    #[default]
    Init,
    Explorer,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(SceneState = SceneState::Explorer)]
pub enum ExplorerCommsSubState {
    #[default]
    Off,
    Live,
}
