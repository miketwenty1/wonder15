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

#[derive(Resource, PartialEq, Clone, Debug)]
pub struct TextVisi {
    pub visibility: Visibility,
    pub player_toggle: bool,
}

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 32.0, y: 32.0 };
const CHUNK_SIZE: UVec2 = UVec2 { x: 250, y: 250 };
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
    let mut tile_entities = Vec::with_capacity(CHUNK_SIZE.x as usize * CHUNK_SIZE.y as usize);
    // let mut random = rand::thread_rng();

    let map_transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * (TILE_SIZE.x),
        chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * (TILE_SIZE.y),
        0.0,
    ));

    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            total.0 += 1;
            if total.0 % 1_000 == 0 {
                info!("total: {:?}", total);
            }
            //let num = random.gen_range(0..=34);
            // let random_color = ;
            let tile_pos = TilePos { x, y };
            let grid_size = TilemapGridSize { x: 32.0, y: 32.0 };
            let map_type = TilemapType::Square;
            let tile_center = tile_pos.center_in_world(&grid_size, &map_type).extend(1.0);
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(35),
                        color: TileColor(Color::Srgba(get_random_color())),
                        ..Default::default()
                    },
                    Visibility::Visible,
                    YoTile,
                    //Transform::from_translation(tile_center),
                ))
                .id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
            tile_entities.push(tile_entity);

            let ulam_v = ulam::get_value_from_xy(
                (chunk_pos.x * CHUNK_SIZE.x as i32) + tile_pos.x as i32,
                (chunk_pos.y * CHUNK_SIZE.y as i32) + tile_pos.y as i32,
            );

            let font_size: f32 = 14.0 - ulam_v.to_string().len() as f32;
            // commands.spawn((
            //     Text2d::new(format!("{}", ulam_v)),
            //     TextFont {
            //         font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            //         font_size,
            //         font_smoothing: FontSmoothing::AntiAliased,
            //     },
            //     text_visi.visibility,
            //     TileText,
            //     TextColor(Color::WHITE),
            //     TextLayout::new_with_justify(JustifyText::Center),
            //     map_transform * Transform::from_translation(tile_center),
            //     //Adding Aabb to attempt to cull Text2d that isn't on screen (works with sprites as parents, but not sure about TileBundles),
            //     YoMap,
            //     Aabb {
            //         center: Vec3A::ZERO,
            //         half_extents: Vec3A::ZERO,
            //     },
            // ));
        }
    }

    let texture_handle: Handle<Image> = asset_server.load("spritesheet/ss-land-v12.png");
    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size: TilemapGridSize { x: 32.0, y: 32.0 },
            size: CHUNK_SIZE.into(),
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size: TILE_SIZE,
            spacing: TilemapSpacing { x: 2.0, y: 2.0 },
            transform: map_transform,
            render_settings: TilemapRenderSettings {
                render_chunk_size: RENDER_CHUNK_SIZE,
                ..Default::default()
            },
            ..Default::default()
        },
        YoMap,
    ));
    //.add_children(&tile_entities);
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
    text_visi: Res<TextVisi>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut text_visible_event: EventWriter<TextVisibilityEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        // text_visi.player_toggle = !text_visi.player_toggle;
        text_visible_event.send(TextVisibilityEvent::KeyPressToggle);
    }
}

// should be the only system responsible for changing the TextVisi Resource
fn text_visibility_event_reader(
    mut text_visi: ResMut<TextVisi>,
    mut event: EventReader<TextVisibilityEvent>,
    mut text_q: Query<&mut Visibility, With<TileText>>,
    cam_q: Query<&OrthographicProjection, With<Camera>>,
) {
    for e in event.read() {
        let within_zoom_showing_threshold = cam_q.get_single().unwrap().scale < TEXT_ZOOM_THRESHOLD;
        let visi_tog = match e {
            TextVisibilityEvent::ButtonToggle => {
                text_visi.player_toggle = !text_visi.player_toggle;
                text_visi.player_toggle
            }
            TextVisibilityEvent::KeyPressToggle => {
                text_visi.player_toggle = !text_visi.player_toggle;
                text_visi.player_toggle
            }
            TextVisibilityEvent::Zoom => text_visi.player_toggle,
        };
        if within_zoom_showing_threshold && visi_tog {
            if text_visi.visibility != Visibility::Visible {
                text_visi.visibility = Visibility::Visible;
                for mut visi in text_q.iter_mut() {
                    *visi = Visibility::Visible;
                }
            }
        } else if text_visi.visibility != Visibility::Hidden {
            text_visi.visibility = Visibility::Hidden;
            for mut visi in text_q.iter_mut() {
                *visi = Visibility::Hidden;
            }
        }
    }
}
//
// ) {
// for mut v in text_q.iter_mut() {
//     *v = Visibility::Visible;
// }
#[allow(clippy::type_complexity)]
fn despawn_outofrange_chunks(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera>>,
    chunks_query_map: Query<(Entity, &Transform), With<YoMap>>,
    mut chunk_manager: ResMut<ChunkManager>,
    despawn_range: Res<DespawnRange>,
) {
    for camera_transform in camera_query.iter() {
        for (entity, chunk_transform) in chunks_query_map.iter() {
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

pub fn main() {}

#[allow(clippy::too_many_arguments)]
#[wasm_bindgen]
pub fn game15() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Performance chunking PoC"),
                        //present_mode: bevy::window::PresentMode::AutoNoVsync,
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
        .insert_resource(TextVisi {
            visibility: Visibility::Visible,
            player_toggle: true,
        })
        .add_systems(Startup, (startup, fit_canvas_to_parent).chain())
        .add_systems(Update, helpers::camera::movement)
        .add_systems(Update, spawn_chunks_around_camera)
        .add_systems(
            Update,
            (
                despawn_outofrange_chunks,
                toggle_text,
                text_visibility_event_reader,
            ),
        )
        .add_event::<TextVisibilityEvent>()
        .run();
}
//fit_canvas_to_parent
