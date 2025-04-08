use bevy::prelude::*;

use crate::scene::explorer::ecs::state::ExplorerSubState;

use super::{
    component::{CancelExplorerSelectionCartBtn, InspectOrBuyExporerCartBtn},
    event::{ClearSelectedTiles, PreviewTileCart},
};

pub fn cancel_selection_btn(
    interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<CancelExplorerSelectionCartBtn>),
    >,
    mut event: EventWriter<ClearSelectedTiles>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            event.send(ClearSelectedTiles);
        }
    }
}

pub fn inspect_or_buy_selection_btn(
    interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<InspectOrBuyExporerCartBtn>),
    >,
    mut state: ResMut<NextState<ExplorerSubState>>,
    mut event: EventWriter<PreviewTileCart>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            // state.set(ExplorerSubState::Paused);
            // event.send(PreviewTileCart);
        }
    }
}
