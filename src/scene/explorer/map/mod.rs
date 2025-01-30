use bevy::prelude::*;
use chunking_building::{
    despawn_building_outofrange_chunks, despawn_buildings, spawn_building_chunks_around_camera,
};
use chunking_text2d::{
    despawn_text, despawn_text_outofrange_chunks, spawn_text_chunks_around_camera,
};
use resource::MapResPlugin;
use setup::{startup, startup_tilemap};
use state::{BuildingToggleState, InitSpawnTileMapState, TextToggleState};
use swap_tiles::swap_tile_index_reader;
use toggle_building::building_toggle_reader;
use toggle_text::text_toggle_reader;
use zoom_fns::zoom_reader;

mod chunking_building;
mod chunking_text2d;
mod component;
mod hard;
mod resource;
mod setup;
mod state;
mod swap_tiles;
mod toggle_building;
mod toggle_text;
mod zoom_fns;
use crate::scene::{ExplorerRunningZoomSub2State, ExplorerSubState};

pub struct ExplorerMapPlugin;

impl Plugin for ExplorerMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MapResPlugin)
            .init_state::<InitSpawnTileMapState>()
            .init_state::<BuildingToggleState>()
            .init_state::<TextToggleState>()
            .add_systems(
                OnEnter(InitSpawnTileMapState::Running),
                (startup).run_if(run_once),
            )
            .add_systems(
                Update,
                (startup_tilemap).run_if(in_state(InitSpawnTileMapState::Running)),
            )
            .add_systems(
                Update,
                (
                    swap_tile_index_reader,
                    building_toggle_reader,
                    text_toggle_reader,
                    zoom_reader,
                )
                    .run_if(in_state(ExplorerSubState::Running)),
            )
            .add_systems(
                Update,
                (
                    despawn_text_outofrange_chunks,
                    spawn_text_chunks_around_camera,
                )
                    .run_if(
                        in_state(ExplorerRunningZoomSub2State::Close)
                            .and(in_state(TextToggleState::On)),
                    ),
            )
            .add_systems(OnExit(ExplorerRunningZoomSub2State::Close), despawn_text)
            .add_systems(OnEnter(TextToggleState::Off), despawn_text)
            .add_systems(
                Update,
                (
                    despawn_building_outofrange_chunks,
                    spawn_building_chunks_around_camera,
                )
                    .run_if(
                        not(in_state(ExplorerRunningZoomSub2State::Far))
                            .and(in_state(BuildingToggleState::On)),
                    ),
            )
            .add_systems(
                OnEnter(ExplorerRunningZoomSub2State::Far),
                despawn_buildings,
            )
            .add_systems(OnEnter(BuildingToggleState::Off), despawn_buildings);
    }
}
