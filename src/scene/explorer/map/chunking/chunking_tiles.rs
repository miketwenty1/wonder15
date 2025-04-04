use crate::{
    ecs::resource::{BlockchainHeight, WorldOwnedTileMap},
    scene::explorer::{
        ecs::{
            component::SelectedTile,
            hard::{TILE_SIZE, TILE_SPAN_SPAWN_NUMBER, TILE_Z},
            resource::{ChunkTypeNumsRes, DespawnTileRangeRes},
        },
        map::ecs::{
            component::{BaseTile, LandIndexComp, MainBaseTileMap, PlayerTileColorComp, UlamComp},
            hard::{DEFAULT_UNSET_TILE_INDEX, TEXTURE_INDEX_FOR_PLAYER_COLOR, TILE_SPACING},
            resource::ChunkTileManagerRes,
        },
    },
};

use bevy::prelude::*;
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapId, TilemapRenderSettings, TilemapTexture, TilemapType},
    tiles::{TileBundle, TileColor, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle,
};

fn spawn_chunk(
    commands: &mut Commands,
    asset_server: &AssetServer,
    chunk_pos: IVec2,
    current_blockheight: u32,
    chunks: &Res<ChunkTypeNumsRes>,
    world_map: &Res<WorldOwnedTileMap>,
) {
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(chunks.tile.into());
    let mut tile_entities = Vec::with_capacity(chunks.tile.x as usize * chunks.tile.y as usize);

    //let mut random = rand::thread_rng();
    for x in 0..chunks.tile.x {
        for y in 0..chunks.tile.y {
            let tile_pos = TilePos { x, y };
            let ulam_v = ulam::get_value_from_xy(
                (chunk_pos.x * chunks.tile.x as i32) + tile_pos.x as i32,
                (chunk_pos.y * chunks.tile.y as i32) + tile_pos.y as i32,
            );
            if current_blockheight >= ulam_v {
                let map_type = TilemapType::Square;
                let tile_center_noz = tile_pos
                    .center_in_world(&TILE_SIZE.into(), &map_type)
                    .extend(1.0);
                let tile_center = Vec3 {
                    x: tile_center_noz.x,
                    y: tile_center_noz.y,
                    z: TILE_Z,
                };
                let tile_from_owned_map = world_map.map.get(&ulam_v);
                let (land_index, player_color) = match tile_from_owned_map {
                    Some(s) => (s.land_index, s.color),
                    None => (DEFAULT_UNSET_TILE_INDEX, Color::Srgba(Color::BLACK.into())),
                };

                let tile_entity = commands
                    .spawn((
                        TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            texture_index: TileTextureIndex(TEXTURE_INDEX_FOR_PLAYER_COLOR),
                            color: TileColor(player_color),
                            ..Default::default()
                        },
                        BaseTile,
                        SelectedTile(false),
                        UlamComp(ulam_v),
                        PlayerTileColorComp(TileColor(player_color)),
                        LandIndexComp(land_index),
                        Transform::from_translation(tile_center),
                    ))
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
                tile_entities.push(tile_entity);
            }
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * chunks.tile.x as f32 * (TILE_SIZE.x),
        chunk_pos.y as f32 * chunks.tile.y as f32 * (TILE_SIZE.y),
        TILE_Z,
    ));

    let texture_handle: Handle<Image> =
        asset_server.load("spritesheet/ss-land-v12-gimp-96-spaced.png");
    commands
        .entity(tilemap_entity)
        .insert((
            TilemapBundle {
                grid_size: TilemapGridSize {
                    x: TILE_SIZE.x,
                    y: TILE_SIZE.y,
                },
                size: chunks.tile.into(),
                storage: tile_storage,
                texture: TilemapTexture::Single(texture_handle),
                tile_size: TILE_SIZE,
                spacing: TILE_SPACING,
                transform,
                render_settings: TilemapRenderSettings {
                    render_chunk_size: chunks.tile * 2,
                    ..Default::default()
                },
                ..Default::default()
            },
            MainBaseTileMap,
        ))
        .add_children(&tile_entities);
}

fn camera_pos_to_chunk_pos(camera_pos: &Vec2, chunks: &Res<ChunkTypeNumsRes>) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(chunks.tile.x as i32, chunks.tile.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
    camera_pos / (chunk_size * tile_size)
}

pub fn spawn_tile_chunks_around_camera(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkTileManagerRes>,
    asset_server: Res<AssetServer>,
    current_block_height: Res<BlockchainHeight>,
    chunks: Res<ChunkTypeNumsRes>,
    world_map: Res<WorldOwnedTileMap>,
) {
    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy(), &chunks);
        for y in (camera_chunk_pos.y - TILE_SPAN_SPAWN_NUMBER)
            ..(camera_chunk_pos.y + TILE_SPAN_SPAWN_NUMBER)
        {
            for x in (camera_chunk_pos.x - TILE_SPAN_SPAWN_NUMBER)
                ..(camera_chunk_pos.x + TILE_SPAN_SPAWN_NUMBER)
            {
                if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                    chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                    spawn_chunk(
                        &mut commands,
                        &asset_server,
                        IVec2::new(x, y),
                        current_block_height.0,
                        &chunks,
                        &world_map,
                    );
                }
            }
        }
    }
}

pub fn despawn_tile_outofrange_chunks(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera>>,
    chunks_query_map: Query<(Entity, &Transform), With<MainBaseTileMap>>,
    mut chunk_manager: ResMut<ChunkTileManagerRes>,
    despawn_range: Res<DespawnTileRangeRes>,
    chunks: Res<ChunkTypeNumsRes>,
) {
    for camera_transform in camera_query.iter() {
        for (entity, chunk_transform) in chunks_query_map.iter() {
            let chunk_pos = chunk_transform.translation.xy();
            let distance = camera_transform.translation.xy().distance(chunk_pos);
            if distance > despawn_range.0 {
                info!("despawning tiles");
                let x = (chunk_pos.x / (chunks.tile.x as f32 * TILE_SIZE.x)).floor() as i32;
                let y = (chunk_pos.y / (chunks.tile.y as f32 * TILE_SIZE.y)).floor() as i32;
                chunk_manager.spawned_chunks.remove(&IVec2::new(x, y));
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
