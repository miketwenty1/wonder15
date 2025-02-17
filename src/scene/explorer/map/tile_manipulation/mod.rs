use super::ecs::state::{BuildingToggleState, TextToggleState};
use crate::{
    ecs::state::SceneState,
    scene::explorer::ecs::state::{ExplorerRunningZoomSub2State, ExplorerSubState},
};
use bevy::prelude::*;
use despawn_floaters::{despawn_buildings, despawn_text};
use swap_tiles::swap_tile_index_reader;
use toggle_building::building_toggle_reader;
use toggle_text::text_toggle_reader;
use update_tile_sprites::read_tile_update_event_sprites;
use update_tile_text::read_tile_update_event_text;
use update_tile_texture::read_tile_update_event_color;

mod despawn_floaters;
mod swap_tiles;
mod toggle_building;
mod toggle_text;
mod update_tile_sprites;
mod update_tile_text;
mod update_tile_texture;

pub struct ExplorerMapTileManipulationPlugin;

impl Plugin for ExplorerMapTileManipulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                swap_tile_index_reader,
                building_toggle_reader,
                text_toggle_reader,
            )
                .run_if(in_state(ExplorerSubState::Running)),
        )
        .add_systems(OnExit(ExplorerRunningZoomSub2State::Close), despawn_text)
        .add_systems(OnEnter(TextToggleState::Off), despawn_text)
        .add_systems(
            OnEnter(ExplorerRunningZoomSub2State::Far),
            despawn_buildings,
        )
        .add_systems(OnEnter(BuildingToggleState::Off), despawn_buildings)
        .add_systems(
            Update,
            (
                read_tile_update_event_color,
                read_tile_update_event_text,
                read_tile_update_event_sprites,
            )
                .run_if(in_state(SceneState::Explorer)),
        );
    }
}
