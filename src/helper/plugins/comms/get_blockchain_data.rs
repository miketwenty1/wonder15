use crate::{
    ecs::resource::{
        BlockchainDataHeight, GameHeight, GameStaticInputs, TileBlockchainData, TileData,
    },
    helper::{
        server_struct::{BlockchainDataHeightFromDB, GameBlockMapDataHeightFromDB},
        utils::funs::{get_resource_for_tile, hex_str_to_32_bytes},
    },
    scene::explorer::ecs::event::{UpdateWorldBlockchainDataEvent, UpdateWorldMapTilesEvent},
};
use bevy::prelude::*;
use wasm_bindgen_futures::spawn_local;

use super::ecs::{
    event::{GetBlockchainUpdates, GetTileUpdates},
    resource::BlockchainTileUpdateChannel,
    structy::GetTileType,
};

pub fn api_get_blockchain_data(
    channel: Res<BlockchainTileUpdateChannel>,
    api_server: Res<GameStaticInputs>,
    // gametime: Res<UpdateGameTimetamp>,
    // game_height: Res<GameHeight>,
    mut event: EventReader<GetBlockchainUpdates>,
    blockchain_data_height: Res<BlockchainDataHeight>,
) {
    for e in event.read() {
        //info!("send api request for tiles");
        let request_height = e.0;
        //for event in player_move_event_reader.read() {
        //let pool = IoTaskPool::get();
        let cc = channel.tx.clone();
        let server = api_server.server_url.to_owned();

        info!("get blockchain data starting at height {}", request_height);
        spawn_local(async move {
            let api_response_text = reqwest::get(format!(
                "{}/comms/blockchain_data_by_height/{}",
                server, request_height
            ))
            .await;
            match api_response_text {
                Ok(o) => {
                    let inner = o.text().await;
                    match inner {
                        Ok(o_inner) => {
                            cc.try_send(o_inner);
                        }
                        Err(e) => info!("inner error blockchain data {}", e),
                    }
                }
                Err(e) => info!("error for blockchain data {}", e),
            }
        });

        //gametime.ts = Utc::now();

        // api_load_block_state.set(CommsApiBlockLoadState::LoadBlockData);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn api_receive_blockchain_server_tiles_by_height(
    channel: ResMut<BlockchainTileUpdateChannel>,
    //api_timer: Res<ApiPollingTimer>,
    //tile_map: Res<WorldOwnedTileMap>,
    mut update_tile_event: EventWriter<UpdateWorldBlockchainDataEvent>,
    //mut gametime: ResMut<UpdateGameTimetamp>,
    // mut checkpoint_time: ResMut<CheckpointTimetamp>,
    // mut game_height: ResMut<GameHeight>,
    //  blockchain_height: Res<BlockchainHeight>,
    mut get_more_tiles: EventWriter<GetBlockchainUpdates>,
    //mut toast: EventWriter<ToastEvent>,
    //mut despawn_inventory: EventWriter<DespawnInventoryHeights>,
    //mut spawn_inventory: EventWriter<AddInventoryRow>,
    //inventory: Res<UserInventoryBlocks>,
    //mut browser_event: EventWriter<WriteBrowserStorage>,
    mut blockchain_data_height: ResMut<BlockchainDataHeight>,
) {
    if !channel.rx.is_empty() {
        //api_timer.timer.finished() &&
        info!("api receive blockchain data");
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
                let r_block_result: Result<BlockchainDataHeightFromDB, serde_json::Error> =
                    serde_json::from_str::<BlockchainDataHeightFromDB>(&og_r);

                match r_block_result {
                    Ok(server_block_data) => {
                        let server_height = server_block_data.height_checkpoint;
                        blockchain_data_height.0 = server_height;
                        info!(
                            "receive blockchain data checkpoint of {}",
                            blockchain_data_height.0
                        );
                        for block_data in server_block_data.blocks {
                            //let mut new_insert_update = false;

                            let new_td = TileBlockchainData {
                                height: block_data.h as u32,
                                block_hash: hex_str_to_32_bytes(&block_data.x),
                                block_time: block_data.t as u64,
                                block_bits: block_data.b as u32,
                                block_n_tx: block_data.n as u32,
                                block_size: block_data.s as u32,
                                block_fee: block_data.f as u64,
                                block_weight: block_data.w as u64,
                                block_ver: block_data.v as u32,
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
                            update_tile_event.send(UpdateWorldBlockchainDataEvent(new_tile_vec));
                            info!("sending UpdateBlockchainWorldMapTilesEvent");
                            get_more_tiles.send(GetBlockchainUpdates(blockchain_data_height.0));
                        }
                    }
                    Err(e) => {
                        info!("error matching on blockchain data r_block_result: {}", e);
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
                info!("receiving blockchain data tiles: {}", e);
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
