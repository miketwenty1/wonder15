use bevy::prelude::*;
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapId, TilemapRenderSettings, TilemapType},
    tiles::{TileBundle, TilePos, TileStorage},
    TilemapBundle,
};

use crate::{
    ecs::resource::BlockchainHeight,
    scene::explorer::{
        ecs::{
            hard::{BUILDING_Z, TILE_SIZE},
            resource::{ChunkTypeNumsRes, DespawnBuildingRangeRes, SpriteSheetBuildingRes},
        },
        map::ecs::{
            component::ChunkBuildingMapComp, hard::TILE_SPACING, resource::ChunkBuildingManagerRes,
        },
    },
};

fn spawn_chunk(
    commands: &mut Commands,
    //asset_server: &AssetServer,
    chunk_pos: IVec2,
    layout: &Handle<TextureAtlasLayout>,
    texture: &Handle<Image>,
    current_blockheight: u32,
    chunks: &Res<ChunkTypeNumsRes>,
) {
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(chunks.building.into());
    let mut tile_entities =
        Vec::with_capacity(chunks.building.x as usize * chunks.building.y as usize);
    //let mut random = rand::thread_rng();
    for x in 0..chunks.building.x {
        for y in 0..chunks.building.y {
            let tile_pos = TilePos { x, y };
            let ulam_v = ulam::get_value_from_xy(
                (chunk_pos.x * chunks.building.x as i32) + tile_pos.x as i32,
                (chunk_pos.y * chunks.building.y as i32) + tile_pos.y as i32,
            );
            if current_blockheight >= ulam_v {
                let map_type = TilemapType::Square;
                let tile_center_noz = tile_pos
                    .center_in_world(&TILE_SIZE.into(), &map_type)
                    .extend(1.0);
                let tile_center = Vec3 {
                    x: tile_center_noz.x,
                    y: tile_center_noz.y,
                    z: BUILDING_Z,
                };
                let tile_entity = commands
                    .spawn((
                        TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            //  texture_index: TileTextureIndex(num),
                            ..Default::default()
                        },
                        // Visibility::Visible,
                        // YoTile,
                        Transform::from_translation(tile_center),
                    ))
                    .with_children(|parent| {
                        let translation = Vec3::new(31.0, 31.0, 3.0);
                        let transform = Transform {
                            translation,
                            ..Default::default()
                        };

                        parent.spawn((
                            Sprite {
                                color: Color::Srgba(Srgba::WHITE),
                                texture_atlas: Some(TextureAtlas {
                                    layout: layout.clone(),
                                    index: 1,
                                }),
                                image: texture.clone(),
                                ..default()
                            },
                            transform,
                        ));
                    })
                    .id();
                tile_storage.set(&tile_pos, tile_entity);

                tile_entities.push(tile_entity);
            }
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * chunks.building.x as f32 * (TILE_SIZE.x),
        chunk_pos.y as f32 * chunks.building.y as f32 * (TILE_SIZE.y),
        3.0,
    ));

    // let texture_handle: Handle<Image> =
    //     asset_server.load("spritesheet/ss-land-v12-gimp-64-spaced.png");

    commands
        .entity(tilemap_entity)
        .insert((
            TilemapBundle {
                grid_size: TilemapGridSize {
                    x: TILE_SIZE.x,
                    y: TILE_SIZE.y,
                },
                size: chunks.building.into(),
                storage: tile_storage,
                //texture: TilemapTexture::Single(texture_handle),
                tile_size: TILE_SIZE,
                spacing: TILE_SPACING,
                transform,
                render_settings: TilemapRenderSettings {
                    render_chunk_size: chunks.building * 2,
                    ..Default::default()
                },
                ..Default::default()
            },
            ChunkBuildingMapComp,
        ))
        .add_children(&tile_entities);
}

fn camera_pos_to_chunk_pos(camera_pos: &Vec2, chunks: &Res<ChunkTypeNumsRes>) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(chunks.building.x as i32, chunks.building.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
    camera_pos / (chunk_size * tile_size)
}

pub fn spawn_building_chunks_around_camera(
    mut commands: Commands,
    //asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkBuildingManagerRes>,
    texture_atlas_handle_building: Res<SpriteSheetBuildingRes>,
    current_block_height: Res<BlockchainHeight>,
    chunks: Res<ChunkTypeNumsRes>, // text_visi: Res<TileBuildingVisibilityRes>,
) {
    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy(), &chunks);
        for y in (camera_chunk_pos.y - 2)..(camera_chunk_pos.y + 2) {
            for x in (camera_chunk_pos.x - 2)..(camera_chunk_pos.x + 2) {
                if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                    chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                    spawn_chunk(
                        &mut commands,
                        //&asset_server,
                        IVec2::new(x, y),
                        &texture_atlas_handle_building.layout,
                        &texture_atlas_handle_building.texture,
                        current_block_height.0,
                        &chunks,
                        // &text_visi,
                    );
                }
            }
        }
    }
}

pub fn despawn_building_outofrange_chunks(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera>>,
    chunks_query_map: Query<(Entity, &Transform), With<ChunkBuildingMapComp>>,
    mut chunk_manager: ResMut<ChunkBuildingManagerRes>,
    despawn_range: Res<DespawnBuildingRangeRes>,
    chunks: Res<ChunkTypeNumsRes>,
) {
    for camera_transform in camera_query.iter() {
        for (entity, chunk_transform) in chunks_query_map.iter() {
            let chunk_pos = chunk_transform.translation.xy();
            let distance = camera_transform.translation.xy().distance(chunk_pos);
            if distance > despawn_range.0 {
                let x = (chunk_pos.x / (chunks.building.x as f32 * TILE_SIZE.x)).floor() as i32;
                let y = (chunk_pos.y / (chunks.building.y as f32 * TILE_SIZE.y)).floor() as i32;
                chunk_manager.spawned_chunks.remove(&IVec2::new(x, y));
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
