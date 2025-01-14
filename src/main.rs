use bevy::{asset::AssetMetaCheck, input::mouse::MouseWheel, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use canvas::fit_canvas_to_parent;
use rand::{thread_rng, Rng};
use text2d_chunking::{
    despawn_outofrange_chunks, spawn_chunks_around_camera, ChunkManager, ChunkMap,
};
use wasm_bindgen::prelude::wasm_bindgen;

mod canvas;
mod helpers;
mod text2d_chunking;

pub const MAX_BLOCK_HEIGHT: u32 = 1_000_000;
pub const MAP_LENGTH: u32 = 1000;
pub const CHUNK_INIT_LOAD_SIZE: u32 = 10_000;
pub const TEXT_ZOOM_THRESHOLD: f32 = 2.5;
pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 66.0, y: 66.0 };
pub const TILE_SPACING: TilemapSpacing = TilemapSpacing { x: 2.0, y: 2.0 };

pub const CHUNK_SIZE: UVec2 = UVec2 { x: 4, y: 4 };
pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};

#[derive(Resource, Debug)]
pub struct DespawnRange(f32);

// pub const SCALE_FACTOR: f32 = 2.0;
#[derive(Component, Debug)]
pub struct YoMap;

#[derive(Component, Debug)]
pub struct Ulam(u32);

#[derive(Resource, Debug)]
pub struct TotalTilesSpawned(u32);

#[derive(Resource)]
struct BonusSpawnTimer(Timer);

#[derive(Component, Debug)]
struct LandIndex(u32);

#[derive(Component, Debug)]
struct PlayerTileColor(TileColor);

// #[derive(Bundle)]
// pub struct MyTileBundle {
//     pub tile_bundle: TileBundle,
//     custom_extras: TileData,
// }

// #[derive(Resource, Debug)]
// pub struct DespawnRange(f32);

// #[derive(Event, Debug)]
// pub enum TextVisibilityEvent {
//     KeyPressToggle,
//     ButtonToggle,
//     Zoom,
// }

pub fn get_random_color() -> Srgba {
    let mut rng = rand::thread_rng();
    let r: f32 = rng.gen_range(0.0..1.0);
    let g: f32 = rng.gen_range(0.0..1.0);
    let b: f32 = rng.gen_range(0.0..1.0);

    //info!("getting a random color: {}-{}-{}", r, g, b);
    Srgba {
        red: r,
        green: g,
        blue: b,
        alpha: 1.0,
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
}

fn startup_tilemap(
    mut commands: Commands,
    mut tile_storage_q: Query<(Entity, &mut TileStorage), Without<ChunkMap>>,
    mut state: ResMut<NextState<InitState>>,
    mut total_tiles_res: ResMut<TotalTilesSpawned>,
    time: Res<Time>,
    mut timer: ResMut<BonusSpawnTimer>,
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
            state.set(InitState::Done);
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

fn swap_tile_index(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        &mut TileTextureIndex,
        &LandIndex,
        &mut TileColor,
        &PlayerTileColor,
    )>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (mut tile_index, land_index, mut tile_color, player_tile_color) in &mut query.iter_mut()
        {
            if *tile_index == TileTextureIndex(35) {
                *tile_color = TileColor(Color::Srgba(Color::WHITE.into()));
                *tile_index = TileTextureIndex(land_index.0);
            } else {
                *tile_color = player_tile_color.0;
                *tile_index = TileTextureIndex(35);
            }
        }
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum InitState {
    #[default]
    Off,
    LoadTiles,
    Done,
}

pub fn zoom_wheel_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    time: Res<Time>,
    mut cam_query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for mouse_wheel in mouse_wheel_events.read() {
        let zoom_amount = 1.0 * time.delta_secs() * mouse_wheel.y;
        for mut ortho in cam_query.iter_mut() {
            ortho.scale -= zoom_amount;
        }
    }
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
    mut local_val: Local<u32>,
    mut state: ResMut<NextState<InitState>>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
            *local_val += 1;
            if *local_val == 30 {
                state.set(InitState::LoadTiles);
            }
        }
    }
}

fn setup_animation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("spritesheet/gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    // commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        Transform::from_translation(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 6.0,
        }),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

pub fn main() {}

#[wasm_bindgen]
pub fn game15() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: String::from(
                    "Basic Example - Press Space to change Texture and H to show/hide tilemap.",
                ),
                ..Default::default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()).set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }),)
        .init_state::<InitState>()
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, (fit_canvas_to_parent, startup, setup_animation).chain())
        .add_systems(Update, (animate_sprite, swap_tile_index))
        .insert_resource(TotalTilesSpawned(0))
        .insert_resource(BonusSpawnTimer(Timer::from_seconds(
            0.05,
            TimerMode::Repeating,
        )))
        .insert_resource(ChunkManager::default())
        .insert_resource(DespawnRange(CHUNK_SIZE.x as f32 * TILE_SIZE.x * 6.0))
        .add_systems(
            Update,
            (startup_tilemap).run_if(in_state(InitState::LoadTiles)),
        )
        .add_systems(Update, (helpers::camera::movement, zoom_wheel_system))
        .add_systems(Update, (spawn_chunks_around_camera, despawn_outofrange_chunks))
        .run();
}
