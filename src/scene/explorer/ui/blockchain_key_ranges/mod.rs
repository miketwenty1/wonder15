use bevy::prelude::*;
use blockchain_value_keys::spawn_legend_driver;

use super::ecs::state::ColorBlockchainKeySubState;

mod blockchain_value_keys;

pub struct BlockchainKeyRangesPlugin;

impl Plugin for BlockchainKeyRangesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ColorBlockchainKeySubState>()
            .enable_state_scoped_entities::<ColorBlockchainKeySubState>()
            .add_systems(
                OnEnter(ColorBlockchainKeySubState::Fee),
                (spawn_legend_driver).run_if(in_state(ColorBlockchainKeySubState::Fee)),
            )
            .add_systems(
                OnEnter(ColorBlockchainKeySubState::BlockTime),
                (spawn_legend_driver).run_if(in_state(ColorBlockchainKeySubState::BlockTime)),
            )
            .add_systems(
                OnEnter(ColorBlockchainKeySubState::TxCount),
                (spawn_legend_driver).run_if(in_state(ColorBlockchainKeySubState::TxCount)),
            )
            .add_systems(
                OnEnter(ColorBlockchainKeySubState::Weight),
                (spawn_legend_driver).run_if(in_state(ColorBlockchainKeySubState::Weight)),
            )
            .add_systems(
                OnEnter(ColorBlockchainKeySubState::TargetDifficulty),
                (spawn_legend_driver)
                    .run_if(in_state(ColorBlockchainKeySubState::TargetDifficulty)),
            )
            .add_systems(
                OnEnter(ColorBlockchainKeySubState::LeadingZeros),
                (spawn_legend_driver).run_if(in_state(ColorBlockchainKeySubState::LeadingZeros)),
            )
            .add_systems(
                OnEnter(ColorBlockchainKeySubState::ExcessWork),
                (spawn_legend_driver).run_if(in_state(ColorBlockchainKeySubState::ExcessWork)),
            )
            .add_systems(
                OnEnter(ColorBlockchainKeySubState::Version),
                (spawn_legend_driver).run_if(in_state(ColorBlockchainKeySubState::Version)),
            );
    }
}
