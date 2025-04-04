use bevy::prelude::*;
use button_system::{cancel_selection_btn, inspect_or_buy_selection_btn};
use event::{ClearSelectedTiles, PreviewTileCart, RefreshTileCart};
use read_new_selection_event::{
    detect_add_changed_selection_sprite, detect_removed_selection_sprites, refresh_tile_cart,
};
use resource::{TileCart, TileCartItem, TileCartVec, UserPurchaseHistory};
use state::ExplorerRunningCartSub2State;
use ui::{spawn_explorer_buttons_for_tilecart, spawn_selection_info};

mod button_system;
pub mod component;
pub mod event;
pub mod read_new_selection_event;
pub mod resource;
pub mod state;
pub mod ui;

pub struct TileCartPlugin;

impl Plugin for TileCartPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RefreshTileCart>()
            .add_event::<PreviewTileCart>()
            .add_event::<ClearSelectedTiles>()
            .add_sub_state::<ExplorerRunningCartSub2State>()
            .enable_state_scoped_entities::<ExplorerRunningCartSub2State>()
            .init_resource::<TileCart>()
            .init_resource::<TileCartItem>()
            .init_resource::<UserPurchaseHistory>()
            .init_resource::<TileCartVec>()
            .add_systems(
                OnEnter(ExplorerRunningCartSub2State::On),
                (spawn_selection_info, spawn_explorer_buttons_for_tilecart),
            )
            .add_systems(
                Update,
                ((
                    (
                        detect_removed_selection_sprites,
                        detect_add_changed_selection_sprite,
                        cancel_selection_btn,
                    ),
                    (refresh_tile_cart, inspect_or_buy_selection_btn).chain(),
                )
                    .chain(),),
            );
    }
}
