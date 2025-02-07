use super::ecs::state::InitSpawnTileMapState;
use bevy::prelude::*;
use chunking::ExplorerMapChunkingPlugin;
use ecs::{
    resource::MapResPlugin,
    state::{BuildingToggleState, TextToggleState},
};
use init::ExplorerMapInitPlugin;
use tile_manipulation::ExplorerMapTileManipulationPlugin;

mod chunking;
mod ecs;
mod init;
mod tile_manipulation;

pub struct ExplorerMapPlugin;

impl Plugin for ExplorerMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MapResPlugin,
            ExplorerMapChunkingPlugin,
            ExplorerMapInitPlugin,
            ExplorerMapTileManipulationPlugin,
        ))
        .init_state::<InitSpawnTileMapState>()
        .init_state::<BuildingToggleState>()
        .init_state::<TextToggleState>();
    }
}
