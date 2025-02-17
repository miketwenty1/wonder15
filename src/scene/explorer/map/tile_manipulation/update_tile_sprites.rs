use bevy::prelude::*;

use crate::scene::explorer::ecs::event::UpdateWorldMapTilesEvent;

pub fn read_tile_update_event_sprites(mut event: EventReader<UpdateWorldMapTilesEvent>) {
    for e in event.read() {
        info!("need to update sprites");
    }
}
