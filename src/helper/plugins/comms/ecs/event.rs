use bevy::ecs::event::Event;

use super::structy::TileUpdatePattern;

#[derive(Event, Debug)]
pub struct RequestServerGameTiles(pub TileUpdatePattern);

#[derive(Event, Debug)]
pub struct GetBlockchainUpdates(pub u32);
