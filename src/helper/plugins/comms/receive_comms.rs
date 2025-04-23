use bevy::{color::palettes::css::INDIGO, prelude::*};
use serde_json::Value;
use web_sys::console::info;

use crate::helper::plugins::comms::ecs::{
    event::{
        BlockDataEvent, BrowserBlockchainDataEvent, BrowserGameTilesEvent, CheckLnPaymentEvent,
        GameTilesByHeightEvent, GameTilesByTsEvent, LnInvoiceEvent, ServerBlockchainDataEvent,
        UserInventoryEvent,
    },
    structy::MessageTypeExtract,
};

use super::ecs::resource::CommsChannel;

#[allow(clippy::too_many_arguments)]
pub fn receive_comm(
    mut channel: ResMut<CommsChannel>,
    mut browser_blockchain_writer: EventWriter<BrowserBlockchainDataEvent>,
    mut browser_game_tiles_writer: EventWriter<BrowserGameTilesEvent>,
    mut server_blockchain_writer: EventWriter<ServerBlockchainDataEvent>,
    mut game_tiles_by_ts_writer: EventWriter<GameTilesByTsEvent>,
    mut game_tiles_by_height_writer: EventWriter<GameTilesByHeightEvent>,
    mut check_ln_payment_writer: EventWriter<CheckLnPaymentEvent>,
    mut ln_invoice_writer: EventWriter<LnInvoiceEvent>,
    mut user_inventory_writer: EventWriter<UserInventoryEvent>,
    mut block_data_writer: EventWriter<BlockDataEvent>,
) {
    if !channel.rx.is_empty() {
        info!("receive comms data");
        while let Ok(msg) = channel.rx.try_recv() {
            info!("receive comms msg: {}", msg);
            if let Ok(extracted) = serde_json::from_str::<MessageTypeExtract>(&msg) {
                info!("extracted msg type: {}", extracted.msg_type);
                if let Ok(json_val) = serde_json::from_str::<Value>(&msg) {
                    info!("json value: {}", json_val);
                    match extracted.msg_type.as_str() {
                        "BrowserBlockchainData" => {
                            browser_blockchain_writer.write(BrowserBlockchainDataEvent(json_val));
                        }
                        "BrowserGameTiles" => {
                            browser_game_tiles_writer.write(BrowserGameTilesEvent(json_val));
                        }
                        "ServerBlockchainData" => {
                            server_blockchain_writer.write(ServerBlockchainDataEvent(json_val));
                        }
                        "GameTilesByTs" => {
                            game_tiles_by_ts_writer.write(GameTilesByTsEvent(json_val));
                        }
                        "GameTilesByHeight" => {
                            game_tiles_by_height_writer.write(GameTilesByHeightEvent(json_val));
                        }
                        "CheckLnPayment" => {
                            check_ln_payment_writer.write(CheckLnPaymentEvent(json_val));
                        }
                        "LnInvoice" => {
                            ln_invoice_writer.write(LnInvoiceEvent(json_val));
                        }
                        "UserInventory" => {
                            user_inventory_writer.write(UserInventoryEvent(json_val));
                        }
                        "BlockData" => {
                            block_data_writer.write(BlockDataEvent(json_val));
                        }
                        _ => {
                            info!("Unknown message type: {}", extracted.msg_type);
                        }
                    }
                }
            }
        }
    }
}
