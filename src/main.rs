use bevy::{
    math::{Vec3A, Vec3Swizzles},
    prelude::*,
    render::primitives::Aabb,
    utils::HashSet,
};
use bevy_ecs_tilemap::prelude::*;
use canvas::fit_canvas_to_parent;
use rand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;
mod canvas;
mod helpers;
use ulam;
// Press WASD to move the camera around, and watch as chunks spawn/despawn in response.

#[derive(Resource, Debug)]
pub struct TotalTilesSpawned(u32);

#[derive(Resource, Debug)]
pub struct DespawnRange(f32);

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 32.0, y: 32.0 };
//const TILE_SIZE_TEXTURE: TilemapTileSize = TilemapTileSize { x: 34.0, y: 34.0 };
// For this example, don't choose too large a chunk size.
const CHUNK_SIZE: UVec2 = UVec2 { x: 8, y: 8 };
// Render chunk sizes are set to 4 render chunks per user specified chunk.
const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};

fn spawn_chunk(
    commands: &mut Commands,
    asset_server: &AssetServer,
    chunk_pos: IVec2,
    total: &mut ResMut<TotalTilesSpawned>,
) {
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());
    // Spawn the elements of the tilemap.
    let mut random = rand::thread_rng();
    info!("IVec2: {:?}", chunk_pos);
    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            total.0 += 1;
            if total.0 % 1_000 == 0 {
                info!("total: {:?}", total);
            }
            let num = random.gen_range(0..=34);
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(num),
                    ..Default::default()
                })
                .id();

            //tile_entity_cmd.set_parent(tilemap_entity);
            // commands
            //     .entity(tilemap_entity)
            //     .add_child(tile_entity_cmd.id());

            tile_storage.set(&tile_pos, tile_entity);
            // commands
            //     .entity(tile_storage.id())
            //     .insert(TileLabel(label_entity));
            //tile_storage.set(&tile_pos, tile_entity);
            // spawn label
            // let label_entity = commands

            //     .id();
            // commands.entity(tile_entity).insert(TileLabel(label_entity));
            let map_transform = Transform::from_translation(Vec3::new(
                chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * (TILE_SIZE.x),
                chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * (TILE_SIZE.y),
                0.0,
            ));

            let grid_size = TilemapGridSize { x: 32.0, y: 32.0 };
            let map_type = TilemapType::Square;
            let tile_center = tile_pos.center_in_world(&grid_size, &map_type).extend(1.0);
            let transform = map_transform * Transform::from_translation(tile_center);
            let tile_pos = TilePos { x, y };
            // let transform = Transform::from_translation(Vec3::new(
            //     chunk_pos.x as f32 * x as f32 * TILE_SIZE.x,
            //     chunk_pos.y as f32 * y as f32 * TILE_SIZE.y,
            //     5.0,
            // ));
            let text_ent = commands
                .spawn((
                    Text2d::new(format!(
                        "{},{}",
                        (chunk_pos.x * CHUNK_SIZE.x as i32) + tile_pos.x as i32,
                        (chunk_pos.y * CHUNK_SIZE.y as i32) + tile_pos.y as i32
                    )),
                    TextFont {
                        font_size: 7.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(JustifyText::Center),
                    transform,
                    Aabb {
                        center: Vec3A::ZERO,
                        half_extents: Vec3A::ZERO,
                    },
                ))
                .id();
            //text_ent_cmd.insert(;
            //let entt = tile_storage.iter().flatten();

            //commands.entity(tile_entity).insert(TileLabel(text_ent));
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * (TILE_SIZE.x),
        chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * (TILE_SIZE.y),
        0.0,
    ));

    let texture_handle: Handle<Image> = asset_server.load("spritesheet/ss-land-v12.png");
    commands.entity(tilemap_entity).insert(TilemapBundle {
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
    });
}

#[derive(Component)]
struct TileLabel(Entity);

// Generates tile position labels of the form: `(tile_pos.x, tile_pos.y)`
fn spawn_tile_labels(
    mut commands: Commands,
    tilemap_q: Query<(&Transform, &TilemapType, &TilemapGridSize, &TileStorage)>,
    tile_q: Query<&mut TilePos>,
) {
    for (map_transform, map_type, grid_size, tilemap_storage) in tilemap_q.iter() {
        for tile_entity in tilemap_storage.iter().flatten() {
            // let tile_pos = tile_q.get(*tile_entity).unwrap();
            // let tile_center = tile_pos.center_in_world(grid_size, map_type).extend(1.0);
            // let transform = *map_transform * Transform::from_translation(tile_center);

            // let label_entity = commands
            //     .spawn((
            //         Text2d::new(format!("{},{}", tile_pos.x, tile_pos.y)),
            //         TextFont {
            //             font_size: 14.0,
            //             ..default()
            //         },
            //         TextColor(Color::BLACK),
            //         TextLayout::new_with_justify(JustifyText::Center),
            //         transform,
            //     ))
            //     .id();
            // commands
            //     .entity(*tile_entity)
            //     .insert(TileLabel(label_entity));
        }
    }
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
) {
    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        for y in (camera_chunk_pos.y - 2)..(camera_chunk_pos.y + 2) {
            for x in (camera_chunk_pos.x - 2)..(camera_chunk_pos.x + 2) {
                if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                    chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                    spawn_chunk(&mut commands, &asset_server, IVec2::new(x, y), &mut total);
                }
            }
        }
    }
}

fn despawn_outofrange_chunks(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera>>,
    chunks_query: Query<(Entity, &Transform)>,
    mut chunk_manager: ResMut<ChunkManager>,
    despawn_range: Res<DespawnRange>,
) {
    for camera_transform in camera_query.iter() {
        for (entity, chunk_transform) in chunks_query.iter() {
            let chunk_pos = chunk_transform.translation.xy();
            let distance = camera_transform.translation.xy().distance(chunk_pos);
            if distance > despawn_range.0 {
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

pub fn main() {}

#[wasm_bindgen]
pub fn game15() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Basic Chunking Example"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(TilemapPlugin)
        .insert_resource(ChunkManager::default())
        .insert_resource(TotalTilesSpawned(0))
        .insert_resource(DespawnRange(CHUNK_SIZE.x as f32 * TILE_SIZE.x * 12.0))
        .add_systems(Startup, (fit_canvas_to_parent, startup).chain())
        .add_systems(Update, helpers::camera::movement)
        .add_systems(Update, (spawn_chunks_around_camera))
        .add_systems(Update, despawn_outofrange_chunks)
        .run();
}
