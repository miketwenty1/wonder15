use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TileColor, TileTextureIndex};

use crate::{
    helper::utils::funs::get_text_color_per_tile_color,
    scene::explorer::{
        ecs::{event::SwapTilesEvent, resource::CurrentTilesRes},
        map::ecs::component::{AssociatedTileColor, LandIndexComp, PlayerTileColorComp, TileText},
    },
};

pub fn swap_tile_index_reader(
    mut event_r: EventReader<SwapTilesEvent>,
    mut query: Query<(
        &mut TileTextureIndex,
        &LandIndexComp,
        &mut TileColor,
        &PlayerTileColorComp,
    )>,
    mut q_text_color: Query<(&mut TextColor, &AssociatedTileColor), With<TileText>>,
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
                for (mut tile_index, _, mut tile_color, player_tile_color) in query.iter_mut() {
                    if *tile_index != TileTextureIndex(35) {
                        *tile_color = player_tile_color.0;
                        *tile_index = TileTextureIndex(35);
                    }
                }
                for (mut text_color, player_color) in q_text_color.iter_mut() {
                    *text_color = TextColor(get_text_color_per_tile_color(&player_color.0));
                }
                *current_tiles = CurrentTilesRes(SwapTilesEvent::PlayerColor);
            }
            SwapTilesEvent::Land => {
                for (mut tile_index, land_index, mut tile_color, _) in query.iter_mut() {
                    if *tile_index == TileTextureIndex(35) {
                        *tile_color = TileColor(Color::Srgba(Color::WHITE.into()));
                        *tile_index = TileTextureIndex(land_index.0);
                    }
                }
                for (mut text_color, _) in q_text_color.iter_mut() {
                    *text_color = TextColor(Color::WHITE);
                }
                *current_tiles = CurrentTilesRes(SwapTilesEvent::Land);
            }
            SwapTilesEvent::Iter => {
                info!("this shouldnt be reached this was taken care of above for swap_type");
            }
        }
    }
}
