use bevy::prelude::*;

use crate::scene::explorer::ecs::state::ExplorerSubState;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(ExplorerSubState = ExplorerSubState::Running)]
pub enum ColorBlockchainKeySubState {
    #[default]
    Off,
    Fee,
    BlockTime,
    TxCount,
    Byte,
    Weight,
    TargetDifficulty,
    TargetDifficultyDiff,
    LeadingZeros,
    ExcessWork,
    Version,
}
