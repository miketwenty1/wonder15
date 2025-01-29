use bevy::prelude::Component;
use bevy_ecs_tilemap::tiles::TileColor;

#[derive(Component, Debug)]
pub struct MainBaseTileMap;

#[derive(Component, Debug)]
pub struct ChunkTextMapComp;

#[derive(Component, Debug)]
pub struct ChunkBuildingMapComp;

#[derive(Component, Debug)]
pub struct UlamComp(pub u32);

#[derive(Component, Debug)]
pub struct PlayerTileColorComp(pub TileColor);

#[derive(Component, Debug)]
pub struct LandIndexComp(pub u32);

#[derive(Component, Debug)]
pub struct TileText;

#[derive(Component, Debug)]
pub struct ChunkTile;
