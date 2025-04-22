use bevy::prelude::*;

use super::ecs::resource::{
    BlockMessagesStorageChannel, BlockchainTileUpdateChannel, BrowserGameCheckpointChannel,
    BrowserIndexedDBStorageChannel, CheckInvoiceChannel, GameTileUpdateChannel,
    RequestInvoiceChannel, UserBlockInventoryChannel,
};

pub fn init_js_comms_channels(mut commands: Commands) {
    let (tx, rx) = async_channel::bounded(4);
    commands.insert_resource(GameTileUpdateChannel { tx, rx });
    let (tx, rx) = async_channel::bounded(4);
    commands.insert_resource(BlockchainTileUpdateChannel { tx, rx });
    let (tx, rx) = async_channel::bounded(1);
    commands.insert_resource(RequestInvoiceChannel { tx, rx });
    let (tx, rx) = async_channel::bounded(1);
    commands.insert_resource(CheckInvoiceChannel { tx, rx });
    let (tx, rx) = async_channel::bounded(1);
    commands.insert_resource(UserBlockInventoryChannel { tx, rx });
    // let (tx, rx) = async_channel::bounded(1);
    // commands.insert_resource(BrowserGameCheckpointChannel { tx, rx });
    let (tx, rx) = async_channel::bounded(10);
    commands.insert_resource(BlockMessagesStorageChannel { tx, rx });
    let (tx, rx) = async_channel::bounded(10);
    commands.insert_resource(BrowserIndexedDBStorageChannel { tx, rx });
}
