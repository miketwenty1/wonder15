use bevy::prelude::*;

use crate::{
    ecs::resource::WorldOwnedTileMap,
    helper::utils::funs::{get_text_color_per_tile_color, vec_tile_updates_to_hashmap},
    scene::explorer::{
        ecs::{
            event::{SwapTilesEvent, UpdateWorldMapTilesEvent},
            resource::CurrentTilesRes,
        },
        map::ecs::component::UlamComp,
    },
};

pub fn read_tile_update_event_text(
    mut event: EventReader<UpdateWorldMapTilesEvent>,
    mut query: Query<(&mut TextColor, &UlamComp)>,
    current_tiles: Res<CurrentTilesRes>,
) {
    for e in event.read() {
        // let mew_tiles_map = vec_tile_updates_to_hashmap(e.0.clone());
        // for (mut text_color, height) in query.iter_mut() {
        //     let text_color_new = match mew_tiles_map.get(&height.0) {
        //         Some(s) => match current_tiles.0 {
        //             SwapTilesEvent::PlayerColor => get_text_color_per_tile_color(&s.color),
        //             _ => Color::WHITE,
        //         },
        //         None => Color::WHITE,
        //     };
        //     text_color.0 = text_color_new;
        // }
    }
}
