use super::ecs::state::InitSpawnMapState;
use bevy::prelude::*;
use chunking::ExplorerMapChunkingPlugin;
use ecs::{
    resource::MapResPlugin,
    state::{BuildingToggleState, TextToggleState},
};
use init::ExplorerMapInitPlugin;
use tile_manipulation::ExplorerMapTileManipulationPlugin;
use world_map::ExplorerMapWorldMapPlugin;

mod chunking;
pub mod ecs;
mod init;
mod tile_manipulation;
mod world_map;

pub struct ExplorerMapPlugin;

impl Plugin for ExplorerMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MapResPlugin,
            ExplorerMapChunkingPlugin,
            ExplorerMapInitPlugin,
            ExplorerMapTileManipulationPlugin,
            ExplorerMapWorldMapPlugin,
        ))
        .init_state::<InitSpawnMapState>()
        .init_state::<BuildingToggleState>()
        .init_state::<TextToggleState>();
    }
}
