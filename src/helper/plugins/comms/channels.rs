use bevy::prelude::*;

use super::ecs::resource::CommsChannel;

pub fn init_js_comms_channels(mut commands: Commands) {
    let (tx, rx) = async_channel::bounded(4);
    commands.insert_resource(CommsChannel { tx, rx });
}
