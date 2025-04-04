use bevy::prelude::*;
use wasm_bindgen::JsValue;

use crate::{
    ecs::resource::WorldOwnedTileMap, helper::plugins::comms::ecs::resource::GameTimetamp,
    scene::explorer::ecs::state::InitSpawnMapState,
};

use super::event::WriteGameTilesIdb;

pub fn write_game_tiles(
    mut event: EventReader<WriteGameTilesIdb>,
    tile_map: Res<WorldOwnedTileMap>,
    gametime: Res<GameTimetamp>,
) {
    for _e in event.read() {
        info!("received event for local write");
        let event_map = web_sys::CustomEventInit::new();
        let event_ts = web_sys::CustomEventInit::new();
        let trim_browser_tile = tile_map.trim_for_browser_storage();
        let www = serde_json::to_string(&trim_browser_tile).expect("world map is a string");
        let map_json_val = &JsValue::from_str(&www);
        let ts_json_val = &JsValue::from_str(&gametime.ts.unwrap().to_string());
        info!("ts to be inserted {:#?}", ts_json_val);
        event_map.set_detail(map_json_val);
        event_ts.set_detail(ts_json_val);
        let mapdata_event =
            web_sys::CustomEvent::new_with_event_init_dict("localbrowserstorage", &event_map);

        info!(
            "writing to storage, the map size is: {:?}",
            tile_map.map.len()
        );
        let ts_event = web_sys::CustomEvent::new_with_event_init_dict("mapcheckpoint", &event_ts);

        if let Ok(o) = ts_event {
            if let Some(window) = web_sys::window() {
                let _ = window.dispatch_event(&o);
                info!("localbrowserstorage attempted");
            } else {
                info!("localbrowserstorage else");
            }
        }

        if let Ok(o) = mapdata_event {
            if let Some(window) = web_sys::window() {
                let _ = window.dispatch_event(&o);
                info!("localbrowserstorage attempted");
            } else {
                info!("localbrowserstorage else");
            }
        }
    }
}
