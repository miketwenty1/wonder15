use bevy::prelude::*;
use chrono::{NaiveDateTime, Timelike};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::js_sys;

use crate::{
    ecs::{
        resource::{TrimGameTileForIdb, WorldOwnedTileMap},
        state::ExplorerCommsSubState,
    },
    helper::{
        plugins::comms::ecs::{
            event::RequestServerGameTiles,
            resource::{
                BrowserGameCheckpointChannel, BrowserIndexedDBStorageChannel, GameTimetamp,
            },
            structy::TileUpdatePattern,
        },
        utils::funs::str_to_dateutc,
    },
    scene::explorer::ecs::{event::UpdateWorldMapTilesEvent, state::InitSpawnMapState},
};

use super::{event::ReadGameTilesIdb, resource::BrowserPollingTimer, state::BrowserStorageState};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn retrieveLocalBrowserGameData() -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = window)]
    fn retrieveCheckpoint() -> js_sys::Promise;
}

pub fn request_local_storage(
    mut event: EventReader<ReadGameTilesIdb>,
    map_channel: Res<BrowserIndexedDBStorageChannel>,
    checkpoint_channel: Res<BrowserGameCheckpointChannel>,
) {
    for _e in event.read() {
        info!("request local storage");
        let map_cc = map_channel.tx.clone();
        let checkpoint_cc = checkpoint_channel.tx.clone();
        spawn_local(async move {
            let mapdata_promise = retrieveLocalBrowserGameData();
            let checkpoint_promise = retrieveCheckpoint();
            let checkpoint_result = JsFuture::from(checkpoint_promise).await;
            let mapdata_result = JsFuture::from(mapdata_promise).await;

            match checkpoint_result {
                Ok(o) => {
                    // info!("good checkpoint {:#?}", o);
                    let _ = checkpoint_cc.try_send(o.as_string().unwrap_or_default());
                }
                Err(e) => {
                    info!("error from checkpoint local browser storage {:#?}", e);
                    let _ = checkpoint_cc.try_send("errorornotfound".to_string());
                }
            }
            match mapdata_result {
                Ok(o) => {
                    // info!("good mapdata {:#?}", o);
                    let _ = map_cc.try_send(o.as_string().unwrap_or_default());
                }
                Err(e) => {
                    info!("error from mapdata local browser storage {:#?}", e);
                    let _ = map_cc.try_send("errorornotfound".to_string());
                }
            }
        });
        info!("read local storage");
    }
}

#[allow(clippy::too_many_arguments)]
pub fn readcheck_game_tiles(
    map_channel: Res<BrowserIndexedDBStorageChannel>,
    checkpoint_channel: Res<BrowserGameCheckpointChannel>,
    browser_poll_timer: Res<BrowserPollingTimer>,
    mut request_tiles_event: EventWriter<RequestServerGameTiles>,
    mut tile_map: ResMut<WorldOwnedTileMap>,
    mut browser_state: ResMut<NextState<BrowserStorageState>>,
    mut game_time: ResMut<GameTimetamp>,
    // mut checkpoint_time: ResMut<CheckpointTimetamp>,
    mut update_tile_event: EventWriter<UpdateWorldMapTilesEvent>,
    mut init_spawn_map: ResMut<NextState<InitSpawnMapState>>,
    mut comms_state: ResMut<NextState<ExplorerCommsSubState>>,
) {
    if browser_poll_timer.timer.just_finished() {
        info!("ticky boy");
        let map_res = map_channel.rx.try_recv();
        let checkpoint_res = checkpoint_channel.rx.try_recv();

        match map_res {
            Ok(o) => {
                // do something here if the data exist
                //info!("checkpoint_res: {:#?}, map_res: {:#?}", checkpoint_res, o);
                if o == "errorornotfound" {
                    request_tiles_event.send(RequestServerGameTiles(TileUpdatePattern::Height));
                } else {
                    let r_result = serde_json::from_str::<TrimGameTileForIdb>(&o); //WorldOwnedTileMap
                    match r_result {
                        Ok(o) => {
                            let world_map_converted = o.convert_trim_to_tilemap();

                            match checkpoint_res {
                                Ok(o) => {
                                    // info!("this is the string for the date: {}", o);
                                    info!("checkpoint string read is: {}", o);
                                    //2024-02-10 06:44:34.499 UTC
                                    let dt_utc = str_to_dateutc(o);
                                    game_time.ts = Some(dt_utc);
                                    *tile_map = world_map_converted.clone();
                                    browser_state.set(BrowserStorageState::Off);
                                    init_spawn_map.set(InitSpawnMapState::Done);
                                    comms_state.set(ExplorerCommsSubState::Live);
                                    request_tiles_event
                                        .send(RequestServerGameTiles(TileUpdatePattern::Ts));

                                    let tiles = world_map_converted.to_tiledata_vec();

                                    update_tile_event.send(UpdateWorldMapTilesEvent(tiles));
                                }
                                Err(e) => {
                                    info!("oh no browser pull 3, {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            info!("some error with parsing browser storage {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                info!("probably don't have any browser storage, if this is a fresh session ignore this, otherwise: {}", e);
                browser_state.set(BrowserStorageState::Off);
            }
        }
    }
}
