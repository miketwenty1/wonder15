#[allow(clippy::too_many_arguments)]
pub fn api_receive_game_server_tiles_by_height_or_ts(
    channel: ResMut<GameTileUpdateChannel>,
    mut update_tile_event: EventWriter<UpdateWorldMapTilesEvent>,
    mut get_more_tiles: EventWriter<RequestServerGameTiles>,
    mut game_height: ResMut<GameHeight>,
    mut game_ts: ResMut<GameTimetamp>,
    mut game_tiles_browser_write: EventWriter<WriteGameTilesIdb>,
    mut should_write_local: Local<bool>,
) {
    if !channel.rx.is_empty() {
        //api_timer.timer.finished() &&
        info!("api receive tiles");
        //info!("checking for tiles response");
        let api_res = channel.rx.try_recv();
        match api_res {
            Ok(og_r) => {
                let mut new_tile_vec = Vec::new();
                let r_block_result: Result<GameBlockDataFromDB, serde_json::Error> =
                    serde_json::from_str::<GameBlockDataFromDB>(&og_r);

                match r_block_result {
                    Ok(server_block_data) => {
                        let height_checkpoint = server_block_data.height_checkpoint;
                        let ts_checkpoint = server_block_data.ts_checkpoint;

                        for block_data in server_block_data.blocks {
                            //let mut new_insert_update = false;
                            let block_hash_as_bytes = hex_str_to_32_bytes(&block_data.block_hash);
                            let resource = get_resource_for_tile(&block_hash_as_bytes);

                            let land_index = resource.spritesheet_index_value();
                            let new_td = TileData {
                                block_hash: block_hash_as_bytes,
                                color: Srgba::hex(block_data.color).unwrap().into(),
                                value: block_data.amount as u32,
                                cost: (block_data.amount * 2) as u32,
                                height: block_data.height as u32,
                                land_index: land_index as u32,
                                resource,
                                ..default()
                            };

                            new_tile_vec.push(new_td.clone());
                        }

                        if !new_tile_vec.is_empty() {
                            update_tile_event.write(UpdateWorldMapTilesEvent(new_tile_vec));
                            info!("sending UpdateWorldMapTilesEvent");
                            if height_checkpoint.is_some() {
                                if game_height.0 == height_checkpoint.unwrap() {
                                    info!("done for now");

                                    if *should_write_local {
                                        info!("writing height trigger local storage");
                                        game_tiles_browser_write.write(WriteGameTilesIdb);
                                    }
                                } else {
                                    info!("getting mo Height son");
                                    get_more_tiles
                                        .write(RequestServerGameTiles(TileUpdatePattern::Height));
                                    game_height.0 = height_checkpoint.unwrap();
                                    *should_write_local = true;
                                }
                            } else if ts_checkpoint.is_some() {
                                if game_ts.ts == ts_checkpoint {
                                    info!("done for now");

                                    if *should_write_local {
                                        info!("writing ts trigger local storage");
                                        game_tiles_browser_write.write(WriteGameTilesIdb);
                                    }
                                } else {
                                    *should_write_local = true;
                                    game_ts.ts = ts_checkpoint;
                                    info!("getting mo Ts son");
                                    get_more_tiles
                                        .write(RequestServerGameTiles(TileUpdatePattern::Ts));
                                }
                            } else {
                                info!("no mo son");
                            }
                        } else {
                            info!("technically i don't think we should ever get here.");
                        }
                    }
                    Err(e) => {
                        info!(
                            "error matching on r_block_result: {}\nissue: {:#?}",
                            e, og_r
                        );
                    }
                }
            }
            Err(e) => {
                info!("receiving tiles: {}", e);
            }
        };
    }
}
