use bevy::prelude::*;

use crate::scene::explorer::ecs::{
    component::{
        BlockTimeToggleBtn, ByteToggleBtn, ExcessWorkToggleBtn, FeeToggleBtn, LeadZerosToggleBtn,
        TgtDiffDiffToggleBtn, TgtDiffToggleBtn, TxCountToggleBtn, VersionToggleBtn,
        WeightToggleBtn,
    },
    event::SwapTilesEvent,
};

use super::ecs::state::ColorBlockchainKeySubState;

pub fn fee_btn(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<FeeToggleBtn>)>,
    mut event: EventWriter<SwapTilesEvent>,
    mut state: ResMut<NextState<ColorBlockchainKeySubState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            event.write(SwapTilesEvent::Fee);
            state.set(ColorBlockchainKeySubState::Fee);
        }
    }
}
pub fn leadzeros_btn(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<LeadZerosToggleBtn>)>,
    mut event: EventWriter<SwapTilesEvent>,
    mut state: ResMut<NextState<ColorBlockchainKeySubState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            event.write(SwapTilesEvent::LeadingZeros);
            state.set(ColorBlockchainKeySubState::LeadingZeros);
        }
    }
}
pub fn tgtdiff_btn(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<TgtDiffToggleBtn>)>,
    mut event: EventWriter<SwapTilesEvent>,
    mut state: ResMut<NextState<ColorBlockchainKeySubState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            event.write(SwapTilesEvent::TargetDifficulty);
            state.set(ColorBlockchainKeySubState::TargetDifficulty);
        }
    }
}
pub fn tgtdiff_diff_btn(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<TgtDiffDiffToggleBtn>)>,
    mut event: EventWriter<SwapTilesEvent>,
    mut state: ResMut<NextState<ColorBlockchainKeySubState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            event.write(SwapTilesEvent::TargetDifficultyDiff);
            state.set(ColorBlockchainKeySubState::TargetDifficultyDiff);
        }
    }
}
pub fn weights_btn(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<WeightToggleBtn>)>,
    mut event: EventWriter<SwapTilesEvent>,
    mut state: ResMut<NextState<ColorBlockchainKeySubState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            event.write(SwapTilesEvent::Weight);
            state.set(ColorBlockchainKeySubState::Weight);
        }
    }
}
pub fn bytes_btn(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ByteToggleBtn>)>,
    mut event: EventWriter<SwapTilesEvent>,
    mut state: ResMut<NextState<ColorBlockchainKeySubState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            event.write(SwapTilesEvent::Byte);
            state.set(ColorBlockchainKeySubState::Byte);
        }
    }
}
pub fn txcount_btn(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<TxCountToggleBtn>)>,
    mut event: EventWriter<SwapTilesEvent>,
    mut state: ResMut<NextState<ColorBlockchainKeySubState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            event.write(SwapTilesEvent::TxCount);
            state.set(ColorBlockchainKeySubState::TxCount);
        }
    }
}
pub fn blocktime_btn(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<BlockTimeToggleBtn>)>,
    mut event: EventWriter<SwapTilesEvent>,
    mut state: ResMut<NextState<ColorBlockchainKeySubState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            event.write(SwapTilesEvent::BlockTime);
            state.set(ColorBlockchainKeySubState::BlockTime);
        }
    }
}
pub fn excesswork_btn(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExcessWorkToggleBtn>)>,
    mut event: EventWriter<SwapTilesEvent>,
    mut state: ResMut<NextState<ColorBlockchainKeySubState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            event.write(SwapTilesEvent::ExcessWork);
            state.set(ColorBlockchainKeySubState::ExcessWork);
        }
    }
}
pub fn version_btn(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<VersionToggleBtn>)>,
    mut event: EventWriter<SwapTilesEvent>,
    mut state: ResMut<NextState<ColorBlockchainKeySubState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            event.write(SwapTilesEvent::Version);
            state.set(ColorBlockchainKeySubState::Version);
        }
    }
}
// ,
// ,
// ,
// ,
// ,
// ,
// excesswork_btn,
// ,
