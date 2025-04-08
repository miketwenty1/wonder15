use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TileColor, TileTextureIndex};

use crate::{
    helper::utils::funs::vec_tile_updates_to_hashmap,
    scene::explorer::{
        ecs::{
            event::{SwapTilesEvent, UpdateWorldMapTilesEvent},
            resource::CurrentTilesRes,
        },
        map::ecs::{
            component::{LandIndexComp, PlayerTileColorComp, UlamComp},
            hard::TEXTURE_INDEX_FOR_PLAYER_COLOR,
        },
    },
};

pub fn read_tile_update_event_color(
    mut event: EventReader<UpdateWorldMapTilesEvent>,
    mut query: Query<(
        &UlamComp,
        &mut TileColor,
        &mut TileTextureIndex,
        &mut LandIndexComp,
        &mut PlayerTileColorComp,
    )>, //
    current_tiles: Res<CurrentTilesRes>,
    // static_inputs: Res<GameStaticInputs>,
) {
    for e in event.read() {
        let mew_tiles_map = vec_tile_updates_to_hashmap(e.0.clone());
        for (
            height,
            mut tile_color,
            mut tile_texture_index,
            mut land_index_store,
            mut tile_color_store,
        ) in query.iter_mut()
        {
            if let Some(s) = mew_tiles_map.get(&height.0) {
                tile_color_store.0 = TileColor(s.color);
                land_index_store.0 = s.land_index;

                match current_tiles.0 {
                    SwapTilesEvent::PlayerColor => {
                        tile_texture_index.0 = TEXTURE_INDEX_FOR_PLAYER_COLOR;
                        *tile_color = TileColor(s.color);
                    }
                    _ => {
                        *tile_color = TileColor(Color::WHITE);
                        tile_texture_index.0 = s.land_index;
                    }
                }
            }
        }
    }
}
