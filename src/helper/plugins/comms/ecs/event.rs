use bevy::ecs::event::Event;

use super::structy::GetTileType;

#[derive(Event, Debug)]
pub struct GetTileUpdates(pub GetTileType);
