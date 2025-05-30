use bevy::prelude::*;
use bevy_ecs_tilemap::{
    anchor::TilemapAnchor,
    map::{TilemapGridSize, TilemapId, TilemapRenderSettings, TilemapSize, TilemapType},
    tiles::{TileBundle, TilePos, TileStorage},
    TilemapBundle,
};

use crate::{
    ecs::resource::WorldOwnedTileMap,
    helper::utils::funs::ulam_to_real_world_xy,
    scene::{
        explorer::{
            ecs::{
                component::BuildingTileComp,
                hard::{BUILDING_CHUNK_SIZE, BUILDING_SPAN_SPAWN_NUMBER, BUILDING_Z, TILE_SIZE},
                resource::{ChunkTypeNumsRes, DespawnBuildingRangeRes, SpriteSheetBuildingRes},
            },
            map::ecs::{
                component::{ChunkBuildingMapComp, RealTileXY, UlamComp},
                hard::TILE_SPACING,
                resource::ChunkBuildingManagerRes,
            },
        },
        initer::ecs::resource::BuildingValueLevelMapper,
    },
};

use super::building_config::spawn_tile_level;

fn spawn_chunk(
    commands: &mut Commands,
    layout: &Handle<TextureAtlasLayout>,
    texture: &Handle<Image>,
    chunk_pos: IVec2,
    world_values: &Res<WorldOwnedTileMap>,
    building_value_mapper: &Res<BuildingValueLevelMapper>,
) {
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(BUILDING_CHUNK_SIZE.into());
    let mut tile_entities =
        Vec::with_capacity(BUILDING_CHUNK_SIZE.x as usize * BUILDING_CHUNK_SIZE.y as usize);
    //let mut random = rand::thread_rng();
    let map_size = TilemapSize {
        x: BUILDING_CHUNK_SIZE.x,
        y: BUILDING_CHUNK_SIZE.y,
    };
    let grid_size = TilemapGridSize {
        x: TILE_SIZE.x,
        y: TILE_SIZE.y,
    };
    let tile_size = TILE_SIZE;
    let map_type = TilemapType::Square;
    let anchor = TilemapAnchor::BottomLeft; //TilemapAnchor::BottomLeft;

    for x in 0..BUILDING_CHUNK_SIZE.x {
        for y in 0..BUILDING_CHUNK_SIZE.y {
            let real_x = (chunk_pos.x * BUILDING_CHUNK_SIZE.x as i32) + x as i32;
            let real_y = (chunk_pos.y * BUILDING_CHUNK_SIZE.y as i32) + y as i32;
            let ulam_v = ulam::get_value_from_xy(real_x, real_y);

            if let Some(s) = world_values.map.get(&ulam_v) {
                let tile_pos = TilePos { x, y };
                let tile_center_noz = tile_pos
                    .center_in_world(&map_size, &grid_size, &tile_size, &map_type, &anchor)
                    .extend(1.0);

                let tile_center_for_tile = Vec3 {
                    x: tile_center_noz.x,
                    y: tile_center_noz.y,
                    z: BUILDING_Z,
                };

                let offset = Vec2 {
                    x: TILE_SIZE.x / 2.,
                    y: TILE_SIZE.y / 2.,
                };
                let tile_center = ulam_to_real_world_xy(ulam_v, TILE_SIZE.x, offset);

                let tile_center = Vec2 {
                    x: tile_center.x,
                    y: tile_center.y,
                };
                let tile_entity = commands
                    .spawn((
                        TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            ..Default::default()
                        },
                        Visibility::Visible,
                        Transform::from_translation(tile_center_for_tile),
                        RealTileXY(tile_center),
                        UlamComp(ulam_v),
                        BuildingTileComp,
                    ))
                    .with_children(|parent| {
                        let level_val = building_value_mapper.get(&s.value).unwrap();
                        spawn_tile_level(level_val, layout, texture, parent, s.color, s.height);
                    })
                    .id();
                tile_storage.set(&tile_pos, tile_entity);

                tile_entities.push(tile_entity);
            }
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * BUILDING_CHUNK_SIZE.x as f32 * (TILE_SIZE.x),
        chunk_pos.y as f32 * BUILDING_CHUNK_SIZE.y as f32 * (TILE_SIZE.y),
        BUILDING_Z,
    ));

    commands
        .entity(tilemap_entity)
        .insert((
            TilemapBundle {
                grid_size,
                size: map_size,
                storage: tile_storage,
                //texture: TilemapTexture::Single(texture_handle),
                tile_size,
                spacing: TILE_SPACING,
                transform,
                anchor,
                render_settings: TilemapRenderSettings {
                    render_chunk_size: BUILDING_CHUNK_SIZE.xy() * 2,
                    ..Default::default()
                },
                ..Default::default()
            },
            ChunkBuildingMapComp,
        ))
        .add_children(&tile_entities);
}

fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(BUILDING_CHUNK_SIZE.x as i32, BUILDING_CHUNK_SIZE.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
    camera_pos / (chunk_size * tile_size)
}
pub fn spawn_building_chunks_around_camera(
    mut commands: Commands,
    //asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkBuildingManagerRes>,
    texture_atlas_handle_building: Res<SpriteSheetBuildingRes>,
    world_map: Res<WorldOwnedTileMap>,
    building_value_mapper: Res<BuildingValueLevelMapper>,
) {
    for transform in camera_query.iter() {
        //let mut spawn_hashmap = HashMap::new();
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        for y in (camera_chunk_pos.y - BUILDING_SPAN_SPAWN_NUMBER)
            ..(camera_chunk_pos.y + BUILDING_SPAN_SPAWN_NUMBER)
        {
            for x in (camera_chunk_pos.x - BUILDING_SPAN_SPAWN_NUMBER)
                ..(camera_chunk_pos.x + BUILDING_SPAN_SPAWN_NUMBER)
            {
                if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                    chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                    spawn_chunk(
                        &mut commands,
                        &texture_atlas_handle_building.layout,
                        &texture_atlas_handle_building.texture,
                        IVec2::new(x, y),
                        &world_map,
                        &building_value_mapper,
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
                info!("despawning buildings");
                let x = (chunk_pos.x / (chunks.building.x as f32 * TILE_SIZE.x)).floor() as i32;
                let y = (chunk_pos.y / (chunks.building.y as f32 * TILE_SIZE.y)).floor() as i32;
                chunk_manager.spawned_chunks.remove(&IVec2::new(x, y));
                commands.entity(entity).despawn();
            }
        }
    }
}
