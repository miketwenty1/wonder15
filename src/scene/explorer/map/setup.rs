use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapId, TilemapSize, TilemapTexture, TilemapType},
    prelude::get_tilemap_center_transform,
    tiles::{TileBundle, TileColor, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle,
};
use rand::{thread_rng, Rng};

use crate::scene::explorer::map::{
    component::{LandIndexComp, MainBaseTileMap, PlayerTileColorComp, UlamComp},
    hard::{TILE_SIZE, TILE_SPACING},
    resource::SpriteSheetBuildingRes,
};

use super::{
    resource::{AdditionalSetupTilesTimerRes, TotalTilesSpawnedRes},
    state::InitSpawnTileMapState,
};

const MAP_LENGTH: u32 = 1000;

const MAX_BLOCK_HEIGHT: u32 = 1_000_000;
const CHUNK_INIT_LOAD_SIZE: u32 = 10_000;

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    //let rounded_map_size = (MAX_BLOCK_HEIGHT as f32).sqrt().ceil() as u32;
    info!("spawning map size: {} by {}", MAP_LENGTH, MAP_LENGTH);
    let map_size = TilemapSize {
        x: MAP_LENGTH + 2,
        y: MAP_LENGTH + 2,
    };
    let tile_storage = TileStorage::empty(map_size);

    let map_type = TilemapType::default();

    let tilemap_entity = commands.spawn_empty().insert(MainBaseTileMap).id();
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

    let building_atlas = TextureAtlasLayout::from_grid(
        bevy::prelude::UVec2::new(32, 32),
        18,
        1,
        Some(bevy::prelude::UVec2::new(2, 2)),
        Some(bevy::prelude::UVec2::new(1, 1)),
    );
    let building_texture_atlas = texture_atlases.add(building_atlas);

    let building_texture = asset_server.load_with_settings(
        "spritesheet/buildings1v2.png",
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        },
    );

    commands.insert_resource(SpriteSheetBuildingRes {
        layout: building_texture_atlas,
        texture: building_texture,
    });
}

pub fn startup_tilemap(
    mut commands: Commands,
    mut tile_storage_q: Query<(Entity, &mut TileStorage), With<MainBaseTileMap>>,
    mut state: ResMut<NextState<InitSpawnTileMapState>>,
    mut total_tiles_res: ResMut<TotalTilesSpawnedRes>,
    time: Res<Time>,
    mut timer: ResMut<AdditionalSetupTilesTimerRes>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }
    let uoff = MAP_LENGTH / 2; // 1000 / 2; // 1000 x 1000
    let previous = total_tiles_res.0;
    let new_destionation = previous + CHUNK_INIT_LOAD_SIZE;
    //info!("previous: {}, destination: {}", previous, new_destionation);
    let mut random = thread_rng();

    for (tilemap_ent, mut tile_storage) in tile_storage_q.iter_mut() {
        if total_tiles_res.0 == MAX_BLOCK_HEIGHT {
            state.set(InitSpawnTileMapState::Done);
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
                    UlamComp(i),
                    PlayerTileColorComp(random_color),
                    LandIndexComp(land_index),
                ))
                .id();

            tile_storage.set(&tile_pos, tile);
            total_tiles_res.0 += 1;
        }
    }
}
