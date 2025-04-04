use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct RefreshTileCart;

#[derive(Event, Debug)]
pub struct PreviewTileCart;

#[derive(Event, Debug)]
pub struct ClearSelectedTiles;
