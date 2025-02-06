use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use init::IniterInitPlugin;

use crate::{ecs::state::SceneState, helper::plugins::comms::CommsPlugin};

mod ecs;
mod init;

pub struct InitScenePlugin;

impl Plugin for InitScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TilemapPlugin, IniterInitPlugin, CommsPlugin))
            .enable_state_scoped_entities::<SceneState>();
    }
}
