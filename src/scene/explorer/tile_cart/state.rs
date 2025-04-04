use bevy::prelude::*;

use crate::scene::explorer::ecs::state::ExplorerSubState;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(ExplorerSubState = ExplorerSubState::Running)]
pub enum ExplorerRunningCartSub2State {
    On,
    #[default]
    Off,
}
