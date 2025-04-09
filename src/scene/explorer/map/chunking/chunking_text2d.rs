use bevy::{math::Vec3A, prelude::*, render::primitives::Aabb, text::FontSmoothing};
use bevy_ecs_tilemap::{
    anchor::TilemapAnchor,
    map::{TilemapGridSize, TilemapId, TilemapRenderSettings, TilemapSize, TilemapType},
    tiles::{TileBundle, TilePos, TileStorage},
    TilemapBundle,
};

use crate::{
    ecs::resource::{BlockchainHeight, WorldOwnedTileMap},
    helper::utils::funs::get_text_color_per_tile_color,
    scene::explorer::{
        ecs::{
            event::SwapTilesEvent,
            hard::{TEXT_CHUNK_SIZE, TEXT_SPAN_SPAWN_NUMBER, TEXT_Z, TILE_SIZE},
            resource::{ChunkTypeNumsRes, CurrentTilesRes, DespawnTextRangeRes},
        },
        map::ecs::{
            component::{AssociatedTileColor, ChunkTextMapComp, TileText, UlamComp},
            hard::TILE_SPACING,
            resource::ChunkTextManagerRes,
        },
    },
};

fn spawn_chunk(
    commands: &mut Commands,
    asset_server: &AssetServer,
    chunk_pos: IVec2,
    current_blockheight: u32,
    world_map: &Res<WorldOwnedTileMap>,
    current_tiles: &Res<CurrentTilesRes>,
) {
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(TEXT_CHUNK_SIZE.into());
    let mut tile_entities =
        Vec::with_capacity(TEXT_CHUNK_SIZE.x as usize * TEXT_CHUNK_SIZE.y as usize);
    //let mut random = rand::thread_rng();
    let map_size = TilemapSize {
        x: TEXT_CHUNK_SIZE.x,
        y: TEXT_CHUNK_SIZE.y,
    };
    let grid_size = TilemapGridSize {
        x: TILE_SIZE.x,
        y: TILE_SIZE.y,
    };
    let tile_size = TILE_SIZE;
    let map_type = TilemapType::Square;
    let anchor = TilemapAnchor::BottomLeft;

    for x in 0..TEXT_CHUNK_SIZE.x {
        for y in 0..TEXT_CHUNK_SIZE.y {
            let tile_pos = TilePos { x, y };
            let ulam_v = ulam::get_value_from_xy(
                (chunk_pos.x * TEXT_CHUNK_SIZE.x as i32) + tile_pos.x as i32,
                (chunk_pos.y * TEXT_CHUNK_SIZE.y as i32) + tile_pos.y as i32,
            );
            if current_blockheight >= ulam_v {
                let tile_center_noz = tile_pos
                    .center_in_world(&map_size, &grid_size, &tile_size, &map_type, &anchor)
                    .extend(1.0);
                let tile_center = Vec3 {
                    x: tile_center_noz.x,
                    y: tile_center_noz.y,
                    z: TEXT_Z,
                };
                let (text_color, player_tile_color) = match world_map.map.get(&ulam_v) {
                    Some(s) => match current_tiles.0 {
                        SwapTilesEvent::PlayerColor => {
                            (get_text_color_per_tile_color(&s.color), s.color)
                        }
                        _ => (Color::WHITE, s.color),
                    },
                    None => (Color::WHITE, Color::BLACK),
                };
                let tile_entity = commands
                    .spawn((
                        TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            //  texture_index: TileTextureIndex(num),
                            ..Default::default()
                        },
                        Visibility::Visible,
                        // YoTile,
                        Transform::from_translation(tile_center),
                    ))
                    .with_children(|parent| {
                        let font_size: f32 = 26.0 - ulam_v.to_string().len() as f32;
                        parent.spawn((
                            Text2d::new(format!("{}", ulam_v)),
                            TextFont {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size,
                                font_smoothing: FontSmoothing::AntiAliased,
                                ..default()
                            },
                            Visibility::Visible,
                            TileText,
                            TextColor(text_color),
                            UlamComp(ulam_v),
                            AssociatedTileColor(player_tile_color),
                            TextLayout::new_with_justify(JustifyText::Center),
                            //Adding Aabb to attempt to cull Text2d that isn't on screen (works with sprites as parents, but not sure about TileBundles),
                            Aabb {
                                center: Vec3A::ZERO,
                                half_extents: Vec3A::ZERO,
                            },
                        ));
                    })
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
                tile_entities.push(tile_entity);
            }
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * TEXT_CHUNK_SIZE.x as f32 * (TILE_SIZE.x),
        chunk_pos.y as f32 * TEXT_CHUNK_SIZE.y as f32 * (TILE_SIZE.y),
        TEXT_Z,
    ));

    commands
        .entity(tilemap_entity)
        .insert((
            TilemapBundle {
                grid_size: TilemapGridSize {
                    x: TILE_SIZE.x,
                    y: TILE_SIZE.y,
                },
                size: TEXT_CHUNK_SIZE.into(),
                storage: tile_storage,
                //texture: TilemapTexture::Single(texture_handle),
                tile_size: TILE_SIZE,
                spacing: TILE_SPACING,
                transform,
                render_settings: TilemapRenderSettings {
                    render_chunk_size: TEXT_CHUNK_SIZE * 2,
                    ..Default::default()
                },
                ..Default::default()
            },
            ChunkTextMapComp,
        ))
        .add_children(&tile_entities);
}

fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(TEXT_CHUNK_SIZE.x as i32, TEXT_CHUNK_SIZE.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
    camera_pos / (chunk_size * tile_size)
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_text_chunk_around_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkTextManagerRes>,
    current_block_height: Res<BlockchainHeight>,
    world_map: Res<WorldOwnedTileMap>,
    current_tiles: Res<CurrentTilesRes>,
) {
    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        for y in (camera_chunk_pos.y - TEXT_SPAN_SPAWN_NUMBER)
            ..(camera_chunk_pos.y + TEXT_SPAN_SPAWN_NUMBER)
        {
            for x in (camera_chunk_pos.x - TEXT_SPAN_SPAWN_NUMBER)
                ..(camera_chunk_pos.x + TEXT_SPAN_SPAWN_NUMBER)
            {
                if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                    chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                    spawn_chunk(
                        &mut commands,
                        &asset_server,
                        IVec2::new(x, y),
                        current_block_height.0,
                        &world_map,
                        &current_tiles,
                    );
                }
            }
        }
    }
}

pub fn despawn_text_outofrange_chunks(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera>>,
    chunks_query_map: Query<(Entity, &Transform), With<ChunkTextMapComp>>,
    mut chunk_manager: ResMut<ChunkTextManagerRes>,
    despawn_range: Res<DespawnTextRangeRes>,
    chunks: Res<ChunkTypeNumsRes>,
) {
    for camera_transform in camera_query.iter() {
        for (entity, chunk_transform) in chunks_query_map.iter() {
            let chunk_pos = chunk_transform.translation.xy();
            let distance = camera_transform.translation.xy().distance(chunk_pos);
            if distance > despawn_range.0 {
                info!("despawning text2d");
                let x = (chunk_pos.x / (chunks.text.x as f32 * TILE_SIZE.x)).floor() as i32;
                let y = (chunk_pos.y / (chunks.text.y as f32 * TILE_SIZE.y)).floor() as i32;
                chunk_manager.spawned_chunks.remove(&IVec2::new(x, y));
                commands.entity(entity).despawn();
            }
        }
    }
}
