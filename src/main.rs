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

#[derive(Resource, Debug)]
pub struct DespawnRange(f32);

#[derive(Event, Debug)]
pub enum TextVisibilityEvent {
    KeyPressToggle,
    ButtonToggle,
    Zoom,
}

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

#[derive(Component)]
struct ComputeTransform(Task<CommandQueue>);

fn startup_tilemap(mut commands: Commands, tile_storage_q: Query<Entity, With<TileStorage>>) {
    let map_size = TilemapSize { x: 1000, y: 1000 };
    for tilemap_ent in tile_storage_q.iter() {
        let thread_pool = AsyncComputeTaskPool::get();
        //let mut random = thread_rng();
        for x in 0..map_size.x - 1 {
            for y in 0..map_size.y - 1 {
                //let num = random.gen_range(0..=34);
                let tile_pos = TilePos { x, y };
                let entity = commands.spawn_empty().id();
                let task = thread_pool.spawn(async move {
                    let mut command_queue = CommandQueue::default();

                    command_queue.push(move |world: &mut World| {
                        let tile_ent = world
                            .entity_mut(entity)
                            .insert(TileBundle {
                                position: tile_pos,
                                tilemap_id: TilemapId(tilemap_ent),
                                texture_index: TileTextureIndex(35),
                                color: TileColor(Color::Srgba(get_random_color())),
                                ..Default::default()
                            })
                            .remove::<ComputeTransform>()
                            .id();

                        let mut system_state: SystemState<(Query<&mut TileStorage>,)> =
                            SystemState::new(world);

                        let mut tile_storage_q = system_state.get_mut(world);

                        let mut tile_storage = tile_storage_q
                            .0
                            .get_single_mut()
                            .expect("tile storage fail");
                        tile_storage.set(&tile_pos, tile_ent);
                    });

                    command_queue
                });
                commands.entity(entity).insert(ComputeTransform(task));
            }
        }
    }
}

fn handle_tasks(
    mut commands: Commands,
    mut transform_tasks: Query<&mut ComputeTransform>,
    //mut tile_storage_q: Query<&mut TileStorage>,
    mut count: Local<u32>,
) {
    // let tile_storage = tile_storage_q.single_mut();
    for mut task in &mut transform_tasks {
        if let Some(mut commands_queue) = block_on(future::poll_once(&mut task.0)) {
            *count += 1;
            // info!("what is the count: {}", *count);
            // append the returned command queue to have it execute later
            commands.append(&mut commands_queue);
        }
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
        .add_systems(Update, (animate_sprite, handle_tasks))
        .add_systems(
            OnEnter(InitState::LoadTiles),
            (startup_tilemap).run_if(run_once),
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
