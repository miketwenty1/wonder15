use bevy::prelude::*;

use crate::{
    ecs::resource::WorldOwnedTileMap, scene::explorer::ecs::event::UpdateWorldMapTilesEvent,
};

pub fn read_game_world_update_event(
    mut event: EventReader<UpdateWorldMapTilesEvent>,
    mut world_map: ResMut<WorldOwnedTileMap>,
) {
    for e in event.read() {
        for tile in e.0.clone() {
            world_map.map.insert(tile.height, tile);
        }
    }
}
