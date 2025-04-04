use bevy::prelude::*;
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapSize, TilemapTexture, TilemapType},
    prelude::get_tilemap_center_transform,
    tiles::TileStorage,
    TilemapBundle,
};

use crate::{
    ecs::resource::FullMapLength,
    scene::explorer::{
        ecs::{hard::TILE_SIZE, state::InitSpawnMapState},
        map::ecs::{component::MainBaseTileMap, hard::TILE_SPACING},
    },
};

pub fn spawn_startup_fullmap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_length: Res<FullMapLength>,
    // mut explorer_sub_state: ResMut<NextState<ExplorerSubState>>,
) {
    //let rounded_map_size = (MAX_BLOCK_HEIGHT as f32).sqrt().ceil() as u32;
    info!("spawning map size: {} by {}", map_length.0, map_length.0);
    let map_size = TilemapSize {
        x: map_length.0,
        y: map_length.0,
    };
    let tile_storage = TileStorage::empty(map_size);

    let map_type = TilemapType::default();

    let tilemap_entity = commands.spawn_empty().insert(MainBaseTileMap).id();
    let texture_handle: Handle<Image> =
        asset_server.load("spritesheet/ss-land-v12-gimp-96-spaced.png");

    let center = get_tilemap_center_transform(&map_size, &TILE_SIZE.into(), &map_type, 0.0);
    // need to do an offset so it lines up with the chunking logic overlay sprites and tiles. Right now it's off by half the distance of a tile in both x/y directions
    // given a 66 pixel tile, the offset would be +33., +33. in for x/y.
    let offset_tran = Vec3::new(
        center.translation.x - (TILE_SIZE.x / 2.), //
        center.translation.y - (TILE_SIZE.y / 2.), //
        0.,
    );
    let transform_for_map = Transform::from_translation(offset_tran);
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TilemapGridSize {
            x: TILE_SIZE.x,
            y: TILE_SIZE.y,
        },
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: TILE_SIZE,
        spacing: TILE_SPACING,
        transform: transform_for_map,
        ..Default::default()
    });

    //explorer_sub_state.set(ExplorerSubState::Running);
}

pub fn spawn_startup_non_fullmap(mut state: ResMut<NextState<InitSpawnMapState>>) {
    info!("non full map, init done");
    state.set(InitSpawnMapState::LocalStorageRead);
}
