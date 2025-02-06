use super::{
    event::RequestTileUpdates,
    resource::{CheckpointTimetamp, GameTileUpdateChannel, UpdateGameTimetamp},
    structy::RequestTileType,
    timer::ApiPollingTimer,
};
use crate::{
    helper::{
        server_struct::GameBlockData,
        utils::funs::{get_land_index, get_resource_for_tile, to_millisecond_precision},
    },
    resource::{BlockchainHeight, GameHeight, GameStaticInputs, TileData, WorldOwnedTileMap},
    scene::explorer::map::event::UpdateTileTextureEvent,
};
use bevy::prelude::*;
use chrono::Duration;
use wasm_bindgen_futures::spawn_local;

pub fn api_get_server_tiles(
    channel: Res<GameTileUpdateChannel>,
    api_server: Res<GameStaticInputs>,
    gametime: Res<UpdateGameTimetamp>,
    game_height: Res<GameHeight>,
    mut event: EventReader<RequestTileUpdates>,
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
            RequestTileType::Height => {
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
            RequestTileType::Ts => {
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
pub fn api_receive_server_tiles(
    channel: ResMut<GameTileUpdateChannel>,
    api_timer: Res<ApiPollingTimer>,
    tile_map: Res<WorldOwnedTileMap>,
    mut update_tile_event: EventWriter<UpdateTileTextureEvent>,
    mut gametime: ResMut<UpdateGameTimetamp>,
    mut checkpoint_time: ResMut<CheckpointTimetamp>,
    mut game_height: ResMut<GameHeight>,
    blockchain_height: Res<BlockchainHeight>,
    mut get_more_tiles: EventWriter<RequestTileUpdates>,
    //mut toast: EventWriter<ToastEvent>,
    //mut despawn_inventory: EventWriter<DespawnInventoryHeights>,
    //mut spawn_inventory: EventWriter<AddInventoryRow>,
    //inventory: Res<UserInventoryBlocks>,
    //mut browser_event: EventWriter<WriteBrowserStorage>,
) {
    if api_timer.timer.finished() && !channel.rx.is_empty() {
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
                let r_block_result = serde_json::from_str::<GameBlockData>(&og_r);

                match r_block_result {
                    Ok(server_block_data) => {
                        // match server_block_data.clone() {
                        //     GameBlockDataFromDBMod {
                        //         ts_checkpoint: Some(t),
                        //         height_checkpoint: None,
                        //         blocks: _,
                        //     } => {
                        //             checkpoint_time.ts = gametime.ts;
                        //             //browser_event.send(WriteBrowserStorage);
                        //             info!("!!!updating game ts to {}", t);
                        //         }
                        //     }
                        //     GameBlockDataFromDBMod {
                        //         ts_checkpoint: None,
                        //         height_checkpoint: Some(h),
                        //         blocks: _,
                        //     } => {
                        //         info!("received height checkpoint {}", h);

                        //         if game_height.0 == h {
                        //             //info!("==");
                        //         } else {
                        //             request_more_height = true;
                        //             //info!("request more height");
                        //         }
                        //         game_height.0 = h;
                        //     }
                        //     _ => println!("Invalid state or both are None"),
                        // }

                        let height_request = server_block_data.height_checkpoint.is_some();
                        let ts_request = server_block_data.ts_checkpoint.is_some();

                        for block_data in server_block_data.blocks {
                            //let mut new_insert_update = false;
                            let resource = get_resource_for_tile(&block_data.hash);
                            let land_index =
                                get_land_index(block_data.height as u32, &resource, None);
                            let new_td = TileData {
                                ln_address: block_data.refund_ln_addr,
                                username: block_data.username,
                                color: Srgba::hex(block_data.color).unwrap().into(),
                                message: block_data.message,
                                value: block_data.amount as u32,
                                cost: (block_data.amount * 2) as u32,
                                height: block_data.height as u32,
                                land_index,
                                event_date: block_data.event_date,
                                resource,
                                block_hash: block_data.hash,
                                block_time: block_data.time,
                                block_bits: block_data.bits,
                                block_n_tx: block_data.n_tx,
                                block_size: block_data.size,
                                block_fee: block_data.fee,
                                block_weight: block_data.weight,
                                block_ver: block_data.ver,
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
                            //info!("api_receive_server_tiles send event UpdateTileTextureEvent, vec size: {}", new_tile_vec.len());
                            update_tile_event.send(UpdateTileTextureEvent(new_tile_vec));
                            if height_request {
                                get_more_tiles.send(RequestTileUpdates(RequestTileType::Height));
                            } else if ts_request {
                                get_more_tiles.send(RequestTileUpdates(RequestTileType::Ts));
                            } else {
                                info!("this is a request type bug, please report");
                            }
                        }

                        // } else {
                        //     api_state.set(CommsApiBlockLoadState::Off);
                        //     // if it's been 15 minutes past last gametime then let's update the browser local storage.
                        //     // this prevents you from needing to update browser cache on every single tile update.
                        //     if gametime.ts - Duration::minutes(15) > checkpoint_time.ts {
                        //         info!("what is the gametime ts?: {}", gametime.ts);
                        //         browser_event.send(WriteBrowserStorage);
                        //     }
                        // }
                    }
                    Err(e) => {
                        info!("");
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
