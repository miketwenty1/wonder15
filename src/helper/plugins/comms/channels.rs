use bevy::prelude::*;

use super::ecs::resource::{
    BlockMessagesStorageChannel, BlockchainMapChannel, BlockchainTileUpdateChannel,
    CheckInvoiceChannel, CheckpointChannel, GameMapChannel, GameTileUpdateChannel,
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
    let (tx, rx) = async_channel::bounded(1);
    // commands.insert_resource(GameMapChannel { tx, rx });
    // let (tx, rx) = async_channel::bounded(1);
    // commands.insert_resource(BlockchainMapChannel { tx, rx });
    // let (tx, rx) = async_channel::bounded(1);
    commands.insert_resource(CheckpointChannel { tx, rx });
    let (tx, rx) = async_channel::bounded(10);
    commands.insert_resource(BlockMessagesStorageChannel { tx, rx });
}
