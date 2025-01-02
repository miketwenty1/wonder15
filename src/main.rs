use bevy::{
    math::{Vec3A, Vec3Swizzles},
    prelude::*,
    render::primitives::Aabb,
    text::FontSmoothing,
    utils::HashSet,
};
use bevy_ecs_tilemap::prelude::*;
use canvas::fit_canvas_to_parent;
use rand::Rng;
mod canvas;
mod helpers;

#[derive(Component, Debug)]
pub struct YoMap;

#[derive(Component, Debug)]
pub struct YoTile;

#[derive(Component, Debug)]
pub struct TileText;

#[derive(Resource, Debug)]
pub struct TotalTilesSpawned(u32);

#[derive(Resource, Debug)]
pub struct DespawnRange(f32);

#[derive(Resource, PartialEq, Clone, Debug)]
pub struct TextVisi(Visibility);

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 32.0, y: 32.0 };
const CHUNK_SIZE: UVec2 = UVec2 { x: 8, y: 8 };
const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};

fn spawn_chunk(
    commands: &mut Commands,
    asset_server: &AssetServer,
    chunk_pos: IVec2,
    total: &mut ResMut<TotalTilesSpawned>,
    text_visi: &Res<TextVisi>,
) {
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());
    let mut random = rand::thread_rng();
    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let map_transform = Transform::from_translation(Vec3::new(
                chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * (TILE_SIZE.x),
                chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * (TILE_SIZE.y),
                0.0,
            ));
            total.0 += 1;
            if total.0 % 1_000 == 0 {
                info!("total: {:?}", total);
            }
            let num = random.gen_range(0..=34);
            let tile_pos = TilePos { x, y };
            let grid_size = TilemapGridSize { x: 32.0, y: 32.0 };
            let map_type = TilemapType::Square;
            let tile_center = tile_pos.center_in_world(&grid_size, &map_type).extend(1.0);
            let transform = map_transform * Transform::from_translation(tile_center);

            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(num),
                        ..Default::default()
                    },
                    YoTile,
                    transform,
                ))
                .with_children(|parent| {
                    let ulam_v = ulam::get_value_from_xy(
                        (chunk_pos.x * CHUNK_SIZE.x as i32) + tile_pos.x as i32,
                        (chunk_pos.y * CHUNK_SIZE.y as i32) + tile_pos.y as i32,
                    );

                    let font_size: f32 = 14.0 - ulam_v.to_string().len() as f32;
                    parent.spawn((
                        Text2d::new(format!("{}", ulam_v)),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size,
                            font_smoothing: FontSmoothing::AntiAliased,
                        },
                        text_visi.0,
                        TileText,
                        TextColor(Color::WHITE),
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
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * (TILE_SIZE.x),
        chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * (TILE_SIZE.y),
        0.0,
    ));

    let texture_handle: Handle<Image> = asset_server.load("spritesheet/ss-land-v12.png");
    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size: TilemapGridSize { x: 32.0, y: 32.0 },
            size: CHUNK_SIZE.into(),
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size: TILE_SIZE,
            spacing: TilemapSpacing { x: 2.0, y: 2.0 },
            transform,
            render_settings: TilemapRenderSettings {
                render_chunk_size: RENDER_CHUNK_SIZE,
                ..Default::default()
            },
            ..Default::default()
        },
        YoMap,
    ));
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
    camera_pos / (chunk_size * tile_size)
}

fn spawn_chunks_around_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut total: ResMut<TotalTilesSpawned>,
    text_visi: Res<TextVisi>,
) {
    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        for y in (camera_chunk_pos.y - 2)..(camera_chunk_pos.y + 2) {
            for x in (camera_chunk_pos.x - 2)..(camera_chunk_pos.x + 2) {
                if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                    chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                    spawn_chunk(
                        &mut commands,
                        &asset_server,
                        IVec2::new(x, y),
                        &mut total,
                        &text_visi,
                    );
                }
            }
        }
    }
}

// Toggling Text doesn't seem to work as expected in terms of performance.
// After many Text2d entites have been spawned, if they are turned off the performance goes down.
fn toggle_text(
    mut text_visi: ResMut<TextVisi>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut text_q: Query<&mut Visibility, With<TileText>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        if *text_visi == TextVisi(Visibility::Visible) {
            *text_visi = TextVisi(Visibility::Hidden);
            for mut v in text_q.iter_mut() {
                *v = Visibility::Hidden;
            }
        } else {
            *text_visi = TextVisi(Visibility::Visible);
            for mut v in text_q.iter_mut() {
                *v = Visibility::Visible;
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn despawn_outofrange_chunks(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera>>,
    chunks_query_map: Query<(Entity, &Transform), With<YoMap>>,
    chunks_query_tiles: Query<(Entity, &Transform), (Without<YoMap>, With<YoTile>)>,
    mut chunk_manager: ResMut<ChunkManager>,
    despawn_range: Res<DespawnRange>,
) {
    for camera_transform in camera_query.iter() {
        // despawning tiles
        for (entity, chunk_transform) in chunks_query_tiles.iter() {
            let chunk_pos = chunk_transform.translation.xy();
            let distance = camera_transform.translation.xy().distance(chunk_pos);
            if distance > despawn_range.0 {
                let x = (chunk_pos.x / (CHUNK_SIZE.x as f32 * TILE_SIZE.x)).floor() as i32;
                let y = (chunk_pos.y / (CHUNK_SIZE.y as f32 * TILE_SIZE.y)).floor() as i32;
                chunk_manager.spawned_chunks.remove(&IVec2::new(x, y));
                commands.entity(entity).despawn_recursive();
            }
        }
        // despawning map with * 2.0 range to avoid panics
        for (entity, chunk_transform) in chunks_query_map.iter() {
            let chunk_pos = chunk_transform.translation.xy();
            let distance = camera_transform.translation.xy().distance(chunk_pos);
            if distance > despawn_range.0 * 2.0 {
                let x = (chunk_pos.x / (CHUNK_SIZE.x as f32 * TILE_SIZE.x)).floor() as i32;
                let y = (chunk_pos.y / (CHUNK_SIZE.y as f32 * TILE_SIZE.y)).floor() as i32;
                chunk_manager.spawned_chunks.remove(&IVec2::new(x, y));
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

#[derive(Default, Debug, Resource)]
struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

pub fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Performance chunking PoC"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(TilemapPlugin)
        .insert_resource(ChunkManager::default())
        .insert_resource(TotalTilesSpawned(0))
        .insert_resource(DespawnRange(CHUNK_SIZE.x as f32 * TILE_SIZE.x * 6.0))
        .insert_resource(TextVisi(Visibility::Visible))
        .add_systems(Startup, (fit_canvas_to_parent, startup).chain())
        .add_systems(Update, helpers::camera::movement)
        .add_systems(Update, spawn_chunks_around_camera)
        .add_systems(Update, (despawn_outofrange_chunks, toggle_text))
        .run();
}
