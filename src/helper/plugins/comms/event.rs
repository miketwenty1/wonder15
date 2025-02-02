use bevy::ecs::event::Event;

use super::structy::RequestTileType;

#[derive(Event, Debug)]
pub struct RequestTileUpdates(pub RequestTileType);
