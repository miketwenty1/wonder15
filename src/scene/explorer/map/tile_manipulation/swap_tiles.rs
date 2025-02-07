use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TileColor, TileTextureIndex};

use crate::scene::explorer::{
    ecs::{event::SwapTilesEvent, resource::CurrentTilesRes},
    map::ecs::component::{LandIndexComp, PlayerTileColorComp},
};

pub fn swap_tile_index_reader(
    mut event_r: EventReader<SwapTilesEvent>,
    mut query: Query<(
        &mut TileTextureIndex,
        &LandIndexComp,
        &mut TileColor,
        &PlayerTileColorComp,
    )>,
    mut current_tiles: ResMut<CurrentTilesRes>,
) {
    for e in event_r.read() {
        let swap_type = if e == &SwapTilesEvent::Iter {
            current_tiles.0.next_tile_swap()
        } else {
            e.clone()
        };

        match swap_type {
            SwapTilesEvent::PlayerColor => {
                for (mut tile_index, _, mut tile_color, player_tile_color) in &mut query.iter_mut()
                {
                    if *tile_index != TileTextureIndex(35) {
                        *tile_color = player_tile_color.0;
                        *tile_index = TileTextureIndex(35);
                    }
                }
                *current_tiles = CurrentTilesRes(SwapTilesEvent::PlayerColor);
            }
            SwapTilesEvent::Land => {
                for (mut tile_index, land_index, mut tile_color, _) in &mut query.iter_mut() {
                    if *tile_index == TileTextureIndex(35) {
                        *tile_color = TileColor(Color::Srgba(Color::WHITE.into()));
                        *tile_index = TileTextureIndex(land_index.0);
                    }
                }
                *current_tiles = CurrentTilesRes(SwapTilesEvent::Land);
            }
            SwapTilesEvent::Iter => {
                info!("this shouldnt be reached this was taken care of above for swap_type");
            }
        }
    }
}
