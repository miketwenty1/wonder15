#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(SceneState = SceneState::Explorer)]
pub enum ExplorerSubState {
    #[default]
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
