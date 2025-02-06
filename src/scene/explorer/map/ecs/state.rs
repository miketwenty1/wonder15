use bevy::prelude::States;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum InitSpawnTileMapState {
    #[default]
    Off,
    Running,
    Done,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum BuildingToggleState {
    #[default]
    On,
    Off,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum TextToggleState {
    #[default]
    On,
    Off,
}
