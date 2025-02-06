use bevy::state::state::States;

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
