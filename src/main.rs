use bevy::{
    ecs::{system::SystemState, world::CommandQueue},
    input::mouse::MouseWheel,
    prelude::*,
    tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, Task},
};
use bevy_ecs_tilemap::prelude::*;
use canvas::fit_canvas_to_parent;
use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::wasm_bindgen;

mod canvas;
mod helpers;

pub const TEXT_ZOOM_THRESHOLD: f32 = 2.5;
#[derive(Component, Debug)]
pub struct YoMap;

#[derive(Component, Debug)]
pub struct YoTile;

#[derive(Component, Debug)]
pub struct TileText;

#[derive(Resource, Debug)]
pub struct TotalTilesSpawned(u32);

#[derive(Resource)]
struct BonusSpawnTimer(Timer);

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

    let map_size = TilemapSize { x: 1000, y: 1000 };
    let tile_storage = TileStorage::empty(map_size);

    let tile_size = TilemapTileSize { x: 34.0, y: 34.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    let tilemap_entity = commands.spawn_empty().id();
    let texture_handle: Handle<Image> = asset_server.load("spritesheet/ss-land-v12.png");

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TilemapGridSize { x: 33.0, y: 33.0 },
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        // spacing: TilemapSpacing { x: 0.0, y: 0.0 },
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

fn startup_tilemap(
    mut commands: Commands,
    tile_storage_q: Query<Entity, With<TileStorage>>,
    mut state: ResMut<NextState<InitState>>,
    mut total_tiles_res: ResMut<TotalTilesSpawned>,
    time: Res<Time>,
    mut timer: ResMut<BonusSpawnTimer>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }
    let moff = 500; // 1000 / 2; // 1000 x 1000
    let chunk_size = 1000;
    let previous = total_tiles_res.0;
    let new_destionation = previous + chunk_size;
    if new_destionation > 1_000_000 {
        state.set(InitState::Off)
    } else {
        let mut veccy = Vec::new();
        for tilemap_ent in tile_storage_q.iter() {
            //let mut random = thread_rng();
            for i in previous..new_destionation {
                let (sx, sy) = ulam::get_xy_from_value(i);
                let x = sx as u32;
                let y = sy as u32;
                let tile_pos = TilePos {
                    x: x + moff,
                    y: y + moff,
                };
                let tile = TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_ent),
                    texture_index: TileTextureIndex(35),
                    color: TileColor(Color::Srgba(get_random_color())),
                    ..Default::default()
                };
                veccy.push(tile);
                total_tiles_res.0 += 1;
            }
        }
        commands.spawn_batch(veccy);
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum InitState {
    #[default]
    Off,
    LoadTiles,
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
        }).set(ImagePlugin::default_nearest()))
        .init_state::<InitState>()
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, (fit_canvas_to_parent, startup, setup_animation).chain())
        .add_systems(Update, (animate_sprite))
        .insert_resource(TotalTilesSpawned(0))
        .insert_resource(BonusSpawnTimer(Timer::from_seconds(
            0.05,
            TimerMode::Repeating,
        )))
        .add_systems(
            Update,
            (startup_tilemap).run_if(in_state(InitState::LoadTiles)),
        )
        .add_systems(Update, (helpers::camera::movement, zoom_wheel_system))
        .run();
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
        Transform::from_scale(Vec3 {
            x: 6.0,
            y: 6.0,
            z: 6.0,
        }),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
