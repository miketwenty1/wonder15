use bevy::prelude::*;

use crate::{
    ecs::resource::WorldBlockchainTileMap,
    scene::explorer::ecs::event::UpdateWorldBlockchainDataEvent,
};

pub fn read_blockchain_world_update_event(
    mut event: EventReader<UpdateWorldBlockchainDataEvent>,
    mut world_map: ResMut<WorldBlockchainTileMap>,
) {
    for e in event.read() {
        for tile in e.0.clone() {
            world_map.map.insert(tile.height, tile);
        }
        info!("world blockchain map size: {}", world_map.map.len());
    }
}
