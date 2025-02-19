use crate::{
    ecs::resource::{BlockchainHeight, GameHeight, GameStaticInputs, TileData, WorldOwnedTileMap},
    helper::{
        server_struct::{GameBlockData, GameBlockMapData, GameBlockMapDataHeightFromDB},
        utils::funs::{get_resource_for_tile, hex_str_to_32_bytes, to_millisecond_precision},
    },
    scene::explorer::ecs::event::UpdateWorldMapTilesEvent,
};
use bevy::prelude::*;
use chrono::Duration;
use wasm_bindgen_futures::spawn_local;

use super::ecs::{
    event::GetTileUpdates,
    resource::{ApiPollingTimer, CheckpointTimetamp, GameTileUpdateChannel, UpdateGameTimetamp},
    structy::GetTileType,
};

pub fn api_get_map_tiles(
    channel: Res<GameTileUpdateChannel>,
    api_server: Res<GameStaticInputs>,
    gametime: Res<UpdateGameTimetamp>,
    game_height: Res<GameHeight>,
    mut event: EventReader<GetTileUpdates>,
) {
    for e in event.read() {
        //info!("send api request for tiles");
        let ts_str = gametime.ts.to_string();
        let game_height = game_height.0;
        //for event in player_move_event_reader.read() {
        //let pool = IoTaskPool::get();
        let cc = channel.tx.clone();
        let server = api_server.server_url.to_owned();
        match e.0 {
            GetTileType::Height => {
                info!("get height tiles sending {}", game_height);
                spawn_local(async move {
                    let api_response_text = reqwest::get(format!(
                        "{}/comms/blockdelta_height/{}",
                        server, game_height
                    ))
                    .await;
                    match api_response_text {
                        Ok(o) => {
                            let inner = o.text().await;
                            match inner {
                                Ok(o_inner) => {
                                    cc.try_send(o_inner);
                                }
                                Err(e) => info!("inner error blockdelta_height {}", e),
                            }
                        }
                        Err(e) => info!("error for blockdelta_height {}", e),
                    }
                });
            }
            GetTileType::Ts => {
                spawn_local(async move {
                    let api_response_r =
                        reqwest::get(format!("{}/comms/blockdelta_ts/{}", server, ts_str)).await;

                    match api_response_r {
                        Ok(o) => {
                            let api_response_text_r = o.text().await;

                            match api_response_text_r {
                                Ok(o) => {
                                    cc.try_send(o);
                                }
                                Err(e) => {
                                    info!("error for request tile ts {:#?}", e);
                                    cc.try_send(e.to_string());
                                }
                            }
                        }
                        Err(e) => {
                            info!("error for request tile ts {:#?}", e);
                            cc.try_send(e.to_string());
                        }
                    }
                });
            }
        }

        //gametime.ts = Utc::now();

        // api_load_block_state.set(CommsApiBlockLoadState::LoadBlockData);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn api_receive_server_tiles_by_height(
    channel: ResMut<GameTileUpdateChannel>,
    //api_timer: Res<ApiPollingTimer>,
    //tile_map: Res<WorldOwnedTileMap>,
    mut update_tile_event: EventWriter<UpdateWorldMapTilesEvent>,
    //mut gametime: ResMut<UpdateGameTimetamp>,
    // mut checkpoint_time: ResMut<CheckpointTimetamp>,
    // mut game_height: ResMut<GameHeight>,
    //  blockchain_height: Res<BlockchainHeight>,
    mut get_more_tiles: EventWriter<GetTileUpdates>,
    //mut toast: EventWriter<ToastEvent>,
    //mut despawn_inventory: EventWriter<DespawnInventoryHeights>,
    //mut spawn_inventory: EventWriter<AddInventoryRow>,
    //inventory: Res<UserInventoryBlocks>,
    //mut browser_event: EventWriter<WriteBrowserStorage>,
    mut game_height: ResMut<GameHeight>,
) {
    if !channel.rx.is_empty() {
        //api_timer.timer.finished() &&
        info!("api receive tiles");
        //info!("checking for tiles response");
        let api_res = channel.rx.try_recv();
        //let mut send_update = false;
        // let ts_request;
        // let height_request;
        // inventory remove vec
        //  let mut remove_inventory_holder: Vec<u32> = Vec::new();
        // let mut add_inventory_holder: Vec<UserGameBlock> = Vec::new();
        match api_res {
            Ok(og_r) => {
                let mut new_tile_vec = Vec::new();
                // for getting a vec of the inventory items needing to be despawned out of inventory of a user.
                // if a tile comes in and the previous owner is the user.. (AND the new owner isn't the user) add to vec.

                //info!("api_receive_server_tiles: {}", r);
                let r_block_result: Result<GameBlockMapDataHeightFromDB, serde_json::Error> =
                    serde_json::from_str::<GameBlockMapDataHeightFromDB>(&og_r);

                match r_block_result {
                    Ok(server_block_data) => {
                        let server_height = match server_block_data.height_checkpoint {
                            Some(s) => s,
                            None => {
                                info!("hit none from server");
                                return;
                            }
                        };
                        game_height.0 = server_height;
                        info!("receive height checkpoint of {}", game_height.0);
                        for block_data in server_block_data.blocks {
                            //let mut new_insert_update = false;
                            let block_hash_as_bytes = hex_str_to_32_bytes(&block_data.block_hash);
                            let resource = get_resource_for_tile(&block_hash_as_bytes);

                            let land_index = resource.spritesheet_index_value();
                            let new_td = TileData {
                                color: Srgba::hex(block_data.color).unwrap().into(),
                                value: block_data.amount as u32,
                                cost: (block_data.amount * 2) as u32,
                                height: block_data.height as u32,
                                land_index: land_index as u32,
                                resource,
                                ..default()
                            };

                            // check if this tile is already in the worldmap as it's coming in.

                            new_tile_vec.push(new_td.clone());
                            //tile_map.map.insert(block_data.height as u32, new_td);

                            // // // // inventory update code

                            // let user_inv_map = &inventory.ownedblocks;
                            // let inv_o = user_inv_map.get(&new_td.height);
                            // if let Some(_s) = inv_o {
                            //     let inv_amount = user_inv_map.get(&new_td.height).unwrap().amount;

                            //     //let aa = new_td.clone();
                            //     #[allow(clippy::comparison_chain)]
                            //     if user_inv_map.contains_key(&new_td.height) {
                            //         if inv_amount < new_td.value {
                            //             info!("need to DEL this from inventory: {}, invamount: {}, checkamount: {}", new_td.height, inv_amount, new_td.value);
                            //             remove_inventory_holder.push(new_td.height);
                            //         } else if inv_amount > new_td.value {
                            //             info!("need to ADD this from inventory: {}, invamount: {}, checkamount: {}", new_td.height, inv_amount, new_td.value);
                            //             add_inventory_holder.push(UserGameBlock {
                            //                 height: new_td.height,
                            //                 amount: new_td.value,
                            //                 color: convert_color_to_hexstring(
                            //                     new_td.color.to_srgba(),
                            //                 ),
                            //             });
                            //         } else {
                            //             info!("block came in and matches inv amount");
                            //         }
                            //     }
                            // }

                            //update_inventory(new_td);
                            // let tile_check = tile_map.map.get(&(block_data.height as u32));
                            // match tile_check {
                            //     Some(existing_tile) => {
                            //         new_td.land_index = existing_tile.land_index;
                            //         if existing_tile != &new_td {
                            //             // if new_td.height == 0 {
                            //             //     info!("A0 is: {:#?}", &new_td);
                            //             // }
                            //             //new_insert_update = true;
                            //             send_update = true;
                            //             new_tile_vec.push(new_td.clone());
                            //         }
                            //     }
                            //     None => {
                            //         // if new_td.height == 0 {
                            //         //     info!("B0 is: {:#?}", &new_td);
                            //         // }
                            //         //new_insert_update = true;
                            //         send_update = true;
                            //         new_tile_vec.push(new_td.clone());
                            //     }
                            // }
                            // if new_insert_update {
                            //     tile_map.map.insert(block_data.height as u32, new_td);
                            // }
                        }

                        //let land_index_map = calculate_index_for_resourced_lands(&mut tile_map.map);
                        //*tile_map = land_index_map;

                        // // // inventory update code
                        // if !remove_inventory_holder.is_empty() {
                        //     despawn_inventory
                        //         .send(DespawnInventoryHeights(remove_inventory_holder));
                        // }
                        // if !add_inventory_holder.is_empty() {
                        //     info!("am i making it to line 243? {:#?}", add_inventory_holder);
                        //     spawn_inventory.send(AddInventoryRow(add_inventory_holder));
                        // }
                        // // // inventory update code

                        if !new_tile_vec.is_empty() {
                            update_tile_event.send(UpdateWorldMapTilesEvent(new_tile_vec));
                            info!("sending UpdateWorldMapTilesEvent");
                            get_more_tiles.send(GetTileUpdates(GetTileType::Height));
                        }
                    }
                    Err(e) => {
                        info!("error matching on r_block_result: {}", e);
                        // if og_r.to_string().contains("logout") {
                        //     logout_user("receive server tiles 1");
                        // } else if !e.to_string().contains("EOF")
                        //     && !e.to_string().contains("empty channel")
                        // {
                        //     if e.to_string().contains("line 1 column 1") {
                        //         toast.send(ToastEvent {
                        //             ttype: ToastType::Bad,
                        //             message: "Seems you lost connection to the server".to_string(),
                        //         });
                        //     } else {
                        //         toast.send(ToastEvent {
                        //             ttype: ToastType::Bad,
                        //             message: format!("error: {}", e),
                        //         });
                        //     }
                        // }
                        // info!("tile receive fail: {}", e);
                    }
                }
                //og_r
            }
            Err(e) => {
                info!("receiving tiles: {}", e);
                // if !e.to_string().contains("EOF") && !e.to_string().contains("empty channel") {
                //     toast.send(ToastEvent {
                //         ttype: ToastType::Bad,
                //         message: e.to_string(),
                //     });
                // }
                // if channel.rx.is_empty() {
                //     api_state.set(CommsApiBlockLoadState::Off);
                // }
                //e.to_string()
            }
        };
    }
}
