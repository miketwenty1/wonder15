use bevy::prelude::Component;
use bevy_ecs_tilemap::tiles::TileColor;

#[derive(Component, Debug)]
pub struct ChunkMapComp;

#[derive(Component, Debug)]
pub struct UlamComp(pub u32);

#[derive(Component, Debug)]
pub struct PlayerTileColorComp(pub TileColor);

#[derive(Component, Debug)]
pub struct LandIndexComp(pub u32);
