use super::{
    event::RequestTileUpdates,
    resource::{GameTileUpdateChannel, UpdateGameTimetamp},
};
use crate::resource::{GameHeight, GameStaticInputs};
use bevy::prelude::*;

pub fn api_get_server_tiles(
    channel: Res<GameTileUpdateChannel>,
    api_server: Res<GameStaticInputs>,
    //mut api_load_block_state: ResMut<NextState<CommsApiBlockLoadState>>,
    gametime: Res<UpdateGameTimetamp>,
    game_height: Res<GameHeight>,
    mut event: EventReader<RequestTileUpdates>,
) {
    for e in event.read() {
        //     //info!("send api request for tiles");
        //     let ts_str = gametime.ts.to_string();
        //     let height_str = init.height.to_string();
        //     //for event in player_move_event_reader.read() {
        //     //let pool = IoTaskPool::get();
        //     let cc = channel.tx.clone();
        //     let server = api_server.0.to_owned();
        //     match e.0 {
        //         RequestTileType::Height => {
        //             info!("get height tiles sending {}", height_str);
        //             spawn_local(async move {
        //                 let api_response_text =
        //                     reqwest::get(format!("{}/comms/blockdelta_height/{}", server, height_str))
        //                         .await;
        //                 match api_response_text {
        //                     Ok(o) => {
        //                         let inner = o.text().await;
        //                         match inner {
        //                             Ok(o_inner) => {
        //                                 cc.try_send(o_inner);
        //                             }
        //                             Err(e) => info!("inner error blockdelta_height {}", e),
        //                         }
        //                     }
        //                     Err(e) => info!("error for blockdelta_height {}", e),
        //                 }
        //             });
        //         }
        //         RequestTileType::Ts => {
        //             spawn_local(async move {
        //                 let api_response_r =
        //                     reqwest::get(format!("{}/comms/blockdelta_ts/{}", server, ts_str)).await;

        //                 match api_response_r {
        //                     Ok(o) => {
        //                         let api_response_text_r = o.text().await;

        //                         match api_response_text_r {
        //                             Ok(o) => {
        //                                 cc.try_send(o);
        //                             }
        //                             Err(e) => {
        //                                 info!("error for request tile ts {:#?}", e);
        //                                 cc.try_send(e.to_string());
        //                             }
        //                         }
        //                     }
        //                     Err(e) => {
        //                         info!("error for request tile ts {:#?}", e);
        //                         cc.try_send(e.to_string());
        //                     }
        //                 }
        //             });
        //         }
        //     }

        //     //gametime.ts = Utc::now();

        //     api_load_block_state.set(CommsApiBlockLoadState::LoadBlockData);
    }
}
