use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use bevy_ecs_tilemap::{
    map::{
        TilemapGridSize, TilemapId, TilemapSize, TilemapSpacing, TilemapTexture, TilemapTileSize,
        TilemapType,
    },
    prelude::get_tilemap_center_transform,
    tiles::{TileBundle, TileColor, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle,
};
use rand::{thread_rng, Rng};

use crate::scene::explorer::map::{
    component::{LandIndex, PlayerTileColor, Ulam},
    resource::SpriteSheetBuilding,
    TILE_SIZE,
};

use super::{
    component::ChunkMap,
    resource::{AdditionalSetupTilesTimer, TotalTilesSpawned},
    InitSpawnTileMap,
};

const MAP_LENGTH: u32 = 5; //1000;
const TILE_SPACING: TilemapSpacing = TilemapSpacing { x: 2.0, y: 2.0 };
const MAX_BLOCK_HEIGHT: u32 = 20; //1_000_000;
const CHUNK_INIT_LOAD_SIZE: u32 = 5; //10_000;

// pub fn startup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
//     mut state: ResMut<NextState<InitSpawnTileMap>>,
// ) {
//     //let rounded_map_size = (MAX_BLOCK_HEIGHT as f32).sqrt().ceil() as u32;
//     info!("spawning map size: {} by {}", MAP_LENGTH, MAP_LENGTH);
//     let map_size = TilemapSize {
//         x: MAP_LENGTH + 2,
//         y: MAP_LENGTH + 2,
//     };
//     let tile_storage = TileStorage::empty(map_size);

//     let map_type = TilemapType::default();

//     let tilemap_entity = commands.spawn_empty().id();
//     let texture_handle: Handle<Image> =
//         asset_server.load("spritesheet/ss-land-v12-gimp-64-spaced.png");

//     let center = get_tilemap_center_transform(&map_size, &TILE_SIZE.into(), &map_type, 0.0);
//     // need to do an offset so it lines up with the chunking logic overlay sprites and tiles. Right now it's off by half the distance of a tile in both x/y directions
//     // given a 66 pixel tile, the offset would be +33., +33. in for x/y.
//     let offset_tran = Vec3::new(
//         center.translation.x + (TILE_SIZE.x / 2.),
//         center.translation.y + (TILE_SIZE.y / 2.),
//         0.,
//     );
//     let transform_for_map = Transform::from_translation(offset_tran);
//     commands.entity(tilemap_entity).insert(TilemapBundle {
//         grid_size: TilemapGridSize {
//             x: TILE_SIZE.x,
//             y: TILE_SIZE.y,
//         },
//         map_type,
//         size: map_size,
//         storage: tile_storage,
//         texture: TilemapTexture::Single(texture_handle),
//         tile_size: TILE_SIZE,
//         spacing: TILE_SPACING,
//         transform: transform_for_map,
//         ..Default::default()
//     });

//     let building_atlas = TextureAtlasLayout::from_grid(
//         bevy::prelude::UVec2::new(32, 32),
//         18,
//         1,
//         Some(bevy::prelude::UVec2::new(2, 2)),
//         Some(bevy::prelude::UVec2::new(1, 1)),
//     );
//     let building_texture_atlas = texture_atlases.add(building_atlas);

//     let building_texture = asset_server.load_with_settings(
//         "spritesheet/buildings1.png",
//         |settings: &mut ImageLoaderSettings| {
//             settings.sampler = ImageSampler::nearest();
//         },
//     );

//     commands.insert_resource(SpriteSheetBuilding {
//         layout: building_texture_atlas,
//         texture: building_texture,
//     });
//     state.set(InitSpawnTileMap::Done);
// }

// pub fn startup_tilemap(
//     mut commands: Commands,
//     mut tile_storage_q: Query<(Entity, &mut TileStorage), Without<ChunkMap>>,
//     mut state: ResMut<NextState<InitSpawnTileMap>>,
//     mut total_tiles_res: ResMut<TotalTilesSpawned>,
//     time: Res<Time>,
//     mut timer: ResMut<AdditionalSetupTilesTimer>,
// ) {
//     if !timer.0.tick(time.delta()).finished() {
//         return;
//     }
//     let uoff = MAP_LENGTH / 2; // 1000 / 2; // 1000 x 1000
//     let previous = total_tiles_res.0;
//     let new_destionation = previous + CHUNK_INIT_LOAD_SIZE;
//     info!("previous: {}, destination: {}", previous, new_destionation);
//     let mut random = thread_rng();

//     for (tilemap_ent, mut tile_storage) in tile_storage_q.iter_mut() {
//         if total_tiles_res.0 == MAX_BLOCK_HEIGHT {
//             state.set(InitSpawnTileMap::Off);
//             return;
//         }
//         for i in previous..new_destionation {
//             let land_index = random.gen_range(0..=34) as u32; // land tile texture index

//             let (sx, sy) = ulam::get_xy_from_value(i);
//             let tile_pos = TilePos {
//                 x: (sx + uoff as i32) as u32,
//                 y: (sy + uoff as i32) as u32,
//             };
//             info!("tile_pos: {:?}", tile_pos);
//             let random_color = if i == 0 {
//                 TileColor(Color::Srgba(Color::WHITE.into()))
//             } else {
//                 TileColor(Color::Srgba(Color::BLACK.into()))
//             };

//             let tile = commands
//                 .spawn((
//                     TileBundle {
//                         position: tile_pos,
//                         tilemap_id: TilemapId(tilemap_ent),
//                         texture_index: TileTextureIndex(35),
//                         color: random_color,
//                         ..Default::default()
//                     },
//                     Ulam(i),
//                     PlayerTileColor(random_color),
//                     LandIndex(land_index),
//                 ))
//                 .id();

//             tile_storage.set(&tile_pos, tile);
//             total_tiles_res.0 += 1;
//         }
//     }
// }

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    //let rounded_map_size = (MAX_BLOCK_HEIGHT as f32).sqrt().ceil() as u32;
    info!("spawning map size: {} by {}", MAP_LENGTH, MAP_LENGTH);
    let map_size = TilemapSize {
        x: MAP_LENGTH + 2,
        y: MAP_LENGTH + 2,
    };
    let tile_storage = TileStorage::empty(map_size);

    let map_type = TilemapType::default();

    let tilemap_entity = commands.spawn_empty().id();
    let texture_handle: Handle<Image> =
        asset_server.load("spritesheet/ss-land-v12-gimp-64-spaced.png");

    let center = get_tilemap_center_transform(&map_size, &TILE_SIZE.into(), &map_type, 0.0);
    // need to do an offset so it lines up with the chunking logic overlay sprites and tiles. Right now it's off by half the distance of a tile in both x/y directions
    // given a 66 pixel tile, the offset would be +33., +33. in for x/y.
    let offset_tran = Vec3::new(
        center.translation.x + (TILE_SIZE.x / 2.),
        center.translation.y + (TILE_SIZE.y / 2.),
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

    //    texture: &Handle<Image>,
    //     layout: &Handle<TextureAtlasLayout>,

    //     texture_atlas_handle_building: Res<SpriteSheetBuilding>,
    let building_atlas = TextureAtlasLayout::from_grid(
        bevy::prelude::UVec2::new(32, 32),
        18,
        1,
        Some(bevy::prelude::UVec2::new(2, 2)),
        Some(bevy::prelude::UVec2::new(1, 1)),
    );
    let building_texture_atlas = texture_atlases.add(building_atlas);

    let building_texture = asset_server.load_with_settings(
        "spritesheet/buildings1.png",
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        },
    );

    commands.insert_resource(SpriteSheetBuilding {
        layout: building_texture_atlas,
        texture: building_texture,
    });
}

pub fn startup_tilemap(
    mut commands: Commands,
    mut tile_storage_q: Query<(Entity, &mut TileStorage), Without<ChunkMap>>,
    mut state: ResMut<NextState<InitSpawnTileMap>>,
    mut total_tiles_res: ResMut<TotalTilesSpawned>,
    time: Res<Time>,
    mut timer: ResMut<AdditionalSetupTilesTimer>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }
    let uoff = MAP_LENGTH / 2; // 1000 / 2; // 1000 x 1000
    let previous = total_tiles_res.0;
    let new_destionation = previous + CHUNK_INIT_LOAD_SIZE;
    info!("previous: {}, destination: {}", previous, new_destionation);
    let mut random = thread_rng();

    for (tilemap_ent, mut tile_storage) in tile_storage_q.iter_mut() {
        if total_tiles_res.0 == MAX_BLOCK_HEIGHT {
            state.set(InitSpawnTileMap::Done);
            return;
        }
        for i in previous..new_destionation {
            let land_index = random.gen_range(0..=34) as u32; // land tile texture index

            let (sx, sy) = ulam::get_xy_from_value(i);
            let tile_pos = TilePos {
                x: (sx + uoff as i32) as u32,
                y: (sy + uoff as i32) as u32,
            };
            let random_color = if i == 0 {
                TileColor(Color::Srgba(Color::WHITE.into()))
            } else {
                TileColor(Color::Srgba(Color::BLACK.into()))
            };

            let tile = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_ent),
                        texture_index: TileTextureIndex(35),
                        color: random_color,
                        ..Default::default()
                    },
                    Ulam(i),
                    PlayerTileColor(random_color),
                    LandIndex(land_index),
                ))
                .id();

            tile_storage.set(&tile_pos, tile);
            total_tiles_res.0 += 1;
        }
    }
}
