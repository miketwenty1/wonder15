use bevy::prelude::*;

use crate::{
    ecs::resource::WorldOwnedTileMap,
    scene::explorer::{
        ecs::hard::MINIMUM_COST_AMOUNT,
        map::ecs::component::{BaseTile, SelectionSprite, UlamComp},
    },
};

use super::{
    component::{BlockCountText, CartPriceText},
    event::RefreshTileCart,
    state::ExplorerRunningCartSub2State,
};

#[allow(clippy::too_many_arguments)]
pub fn refresh_tile_cart(
    mut events: EventReader<RefreshTileCart>,
    mut cart_state: ResMut<NextState<ExplorerRunningCartSub2State>>,
    tile_selected_query: Query<&ChildOf, With<SelectionSprite>>,
    tile_query: Query<&UlamComp, With<BaseTile>>,
    tile_map: Res<WorldOwnedTileMap>,
    mut block_count_q: Query<&mut Text, With<BlockCountText>>,
    mut price_q: Query<&mut Text, (With<CartPriceText>, Without<BlockCountText>)>,
) {
    for _e in events.read() {
        let mut total_cost = 0;
        let mut total_selected = 0;

        for child_of in tile_selected_query.iter() {
            if let Ok(ulam) = tile_query.get(child_of.parent()) {
                info!("found ulam: {}", ulam.0);
                if let Some(tile_data) = tile_map.map.get(&ulam.0) {
                    info!("worldmap found ulam val");
                    total_cost += tile_data.cost;
                } else {
                    total_cost += MINIMUM_COST_AMOUNT;
                }

                total_selected += 1;
            }
        }

        for mut text in &mut block_count_q {
            **text = format!("Blocks Selected: {}", total_selected);
        }

        for mut text in &mut price_q {
            **text = format!("Price: {} satoshis", total_cost);
        }

        // Update cart state
        if total_selected > 0 {
            info!("turning on");
            cart_state.set(ExplorerRunningCartSub2State::On);
        } else {
            info!("turning off");
            cart_state.set(ExplorerRunningCartSub2State::Off);
        }
    }
}

pub fn detect_add_changed_selection_sprite(
    mut event: EventWriter<RefreshTileCart>,
    query: Query<&SelectionSprite, Added<SelectionSprite>>,
) {
    if !query.is_empty() {
        info!("detect_add_changed_selection_sprite - time to refresh");
        event.write(RefreshTileCart);
    }
}

pub fn detect_removed_selection_sprites(
    mut event: EventWriter<RefreshTileCart>,
    mut removed: RemovedComponents<SelectionSprite>,
) {
    for _ in removed.read() {
        info!("detect_removed_selection_sprites - time to refresh");
        event.write(RefreshTileCart);
    }
}
