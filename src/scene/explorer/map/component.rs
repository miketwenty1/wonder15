use bevy::prelude::Component;
use bevy_ecs_tilemap::tiles::TileColor;

#[derive(Component, Debug)]
pub struct ChunkMap;

#[derive(Component, Debug)]
pub struct Ulam(pub u32);

#[derive(Component, Debug)]
pub struct PlayerTileColor(pub TileColor);

#[derive(Component, Debug)]
pub struct LandIndex(pub u32);
