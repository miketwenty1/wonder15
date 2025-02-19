use bevy::prelude::*;

use crate::{
    ecs::resource::WorldOwnedTileMap, scene::explorer::ecs::event::UpdateWorldMapTilesEvent,
};

pub fn read_tile_update_event_text(mut event: EventReader<UpdateWorldMapTilesEvent>) {
    for e in event.read() {
        info!("need to update text");
    }
}
