use bevy::prelude::*;
use blockchain_value_keys::spawn_legend;

use super::ecs::state::ColorBlockchainKeySubState;

mod blockchain_value_keys;

pub struct BlockchainKeyRangesPlugin;

impl Plugin for BlockchainKeyRangesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ColorBlockchainKeySubState>()
            .enable_state_scoped_entities::<ColorBlockchainKeySubState>()
            .add_systems(
                OnEnter(ColorBlockchainKeySubState::Fee),
                (spawn_legend).run_if(in_state(ColorBlockchainKeySubState::Fee)),
            );
    }
}
