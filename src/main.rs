use bevy::{
    ecs::world::CommandQueue,
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

fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component)]
struct PositionXY(TilePos);

#[derive(Resource, Debug)]
struct MapHolder(Entity);

#[derive(Component)]
struct ComputeTransform(Task<CommandQueue>);

fn startup_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut res_tilemap: ResMut<MapHolder>,
) {
    let texture_handle: Handle<Image> = asset_server.load("spritesheet/ss-land-v12.png");
    let map_size = TilemapSize { x: 1000, y: 1000 };
    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();
    *res_tilemap = MapHolder(tilemap_entity);
    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.

    // Spawn the elements of the tilemap.
    // Alternatively, you can use helpers::filling::fill_tilemap.
    let thread_pool = AsyncComputeTaskPool::get();
    let mut random = thread_rng();
    for x in 0..map_size.x - 1 {
        for y in 0..map_size.y - 1 {
            //let num = random.gen_range(0..=34);
            let tile_pos = TilePos { x, y };
            let entity = commands.spawn_empty().id();
            let task = thread_pool.spawn(async move {
                let mut command_queue = CommandQueue::default();

                command_queue.push(move |world: &mut World| {
                    world
                        .entity_mut(entity)
                        .insert(TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            texture_index: TileTextureIndex(35),
                            color: TileColor(Color::Srgba(get_random_color())),
                            ..Default::default()
                        })
                        .remove::<ComputeTransform>();

                    //tile_storage.set(&tile_pos, tile_entity);
                });

                command_queue
            });
            commands.entity(entity).insert(ComputeTransform(task));
            commands.entity(entity).insert(PositionXY(tile_pos));
        }
    }
}
fn handle_tasks(
    mut commands: Commands,
    mut transform_tasks: Query<(&mut ComputeTransform)>,
    mut res_map: ResMut<MapHolder>,
    mut count: Local<u32>,
) {
    for mut task in &mut transform_tasks {
        if let Some(mut commands_queue) = block_on(future::poll_once(&mut task.0)) {
            *count += 1;
            // append the returned command queue to have it execute later
            commands.append(&mut commands_queue);

            if *count % 1000 == 0 {
                info!("what is the count: {}", *count);
            }
        }
    }
}
fn swap_texture_or_hide(
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut TilemapTexture, &mut Visibility)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let texture_a = TilemapTexture::Single(asset_server.load("tiles.png"));
        let texture_b = TilemapTexture::Single(asset_server.load("tiles2.png"));
        for (mut tilemap_tex, _) in &mut query {
            if *tilemap_tex == texture_a {
                *tilemap_tex = texture_b.clone();
            } else {
                *tilemap_tex = texture_a.clone();
            }
        }
    }
    if keyboard_input.just_pressed(KeyCode::KeyH) {
        for (_, mut visibility) in &mut query {
            *visibility = match *visibility {
                Visibility::Inherited | Visibility::Visible => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
            };
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
        .add_systems(Update, swap_texture_or_hide)
        .insert_resource(MapHolder(Entity::from_raw(456456)))
        .run();
}

pub fn zoom_wheel_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    time: Res<Time>,
    mut cam_query: Query<&mut OrthographicProjection, With<Camera>>,
    // mut event_zoom: EventWriter<LdChange>,
    // mut level_of_detail: ResMut<CurrentLd>,
) {
    for mouse_wheel in mouse_wheel_events.read() {
        let zoom_amount = 1.0 * time.delta_secs() * mouse_wheel.y;
        for mut ortho in cam_query.iter_mut() {
            // let previous_ld = level_of_detail.0;
            ortho.scale -= zoom_amount;
            // ortho.scale = ortho.scale.clamp(ZOOM_IN_MAX, ZOOM_OUT_MAX);
            //let current_ld = LevelOfDetail::get_level(ortho.scale);
            // if current_ld != previous_ld {
            //     *level_of_detail = CurrentLd(current_ld);
            //     event_zoom.send(LdChange(current_ld));
            // }
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
            z: 80.0,
        }),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

// let map_size = TilemapSize { x: 1000, y: 1000 };
// let mut tile_storage = TileStorage::empty(map_size);

// let tile_size = TilemapTileSize { x: 34.0, y: 34.0 };
//     let grid_size = tile_size.into();
//     let map_type = TilemapType::default();

//     commands.entity(tilemap_entity).insert(TilemapBundle {
//         grid_size: TilemapGridSize { x: 33.0, y: 33.0 },
//         map_type,
//         size: map_size,
//         storage: tile_storage,
//         texture: TilemapTexture::Single(texture_handle),
//         tile_size,
//         // spacing: TilemapSpacing { x: 0.0, y: 0.0 },
//         transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
//         ..Default::default()
//     });
