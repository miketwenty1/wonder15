use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use init::IniterInitPlugin;

use crate::{
    ecs::state::{ExplorerCommsSubState, SceneState},
    helper::plugins::{browser::BrowserPlugin, comms::CommsPlugin},
};

pub mod ecs;
mod init;

pub struct InitScenePlugin;

impl Plugin for InitScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ExplorerCommsSubState>()
            .add_plugins((TilemapPlugin, IniterInitPlugin, CommsPlugin, BrowserPlugin))
            .enable_state_scoped_entities::<SceneState>();
    }
}
