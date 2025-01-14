// use bevy::prelude::*;

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_systems(Startup, setup);
// }

// fn setup() {}
//! Shows how to render simple primitive shapes with a single color.
//!
//! You can toggle wireframes with the space bar except on wasm. Wasm does not support
//! `POLYGON_MODE_LINE` on the gpu.

use bevy::{input::mouse::MouseWheel, prelude::*};
use canvas::fit_canvas_to_parent;
use rand::Rng;
use spritesheet::{setup_spritesheets, SpriteSheetLand};
use wasm_bindgen::prelude::wasm_bindgen;

mod canvas;
mod spritesheet;

pub const SIZE_OF_SQUARE: f32 = 32.0;
pub const SCALE_FACTOR: f32 = SIZE_OF_SQUARE / 32.0;
pub const MOVE_SPEED: f32 = 400.0;
pub const ZOOM_SPEED: f32 = 0.05;
pub const ZOOM_IN_MAX: f32 = SIZE_OF_SQUARE / 160.0;
pub const ZOOM_OUT_MAX: f32 = 34.0;
pub const SPAWN_COUNT: u32 = 1_000_000;
pub const GRID_SPACING: f32 = SIZE_OF_SQUARE / 32.0;

#[derive(Resource, Debug)]
pub struct CurrentLd(LevelOfDetail);

#[derive(Component, Clone, PartialEq, Copy, Debug, Default)]
pub enum LevelOfDetail {
    #[default]
    Ld0,
    Ld1,
    Ld2,
}

impl LevelOfDetail {
    fn get_level(zoom: f32) -> Self {
        if (8.0..=ZOOM_OUT_MAX).contains(&zoom) {
            LevelOfDetail::Ld2
        } else if (4.0..8.0).contains(&zoom) {
            LevelOfDetail::Ld1
        } else if (ZOOM_IN_MAX..4.0).contains(&zoom) {
            LevelOfDetail::Ld0
        } else {
            panic!("Zoom out of range");
        }
    }

    fn level_name(&self) -> &str {
        match self {
            LevelOfDetail::Ld2 => "Ld2",
            LevelOfDetail::Ld1 => "Ld1",
            LevelOfDetail::Ld0 => "Ld0",
        }
    }
}

#[derive(Event, Debug)]
pub struct LdChange(LevelOfDetail);

pub fn main() {}

#[wasm_bindgen]
pub fn game15() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins,))
        .add_systems(
            Startup,
            (setup_spritesheets, fit_canvas_to_parent, setup).chain(),
        )
        .add_event::<LdChange>()
        .insert_resource(CurrentLd(LevelOfDetail::Ld0))
        .add_systems(
            Update,
            (
                keyboard_movement_camera_system,
                zoom_wheel_system,
                visi_tiles,
            ),
        )
        .run();
}

// // mesh example is too laggy even at 60,000
// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     commands.spawn(Camera2d);
//     let shape = meshes.add(Rectangle::new(SIZE_OF_SQUARE, SIZE_OF_SQUARE));
//     //meshes.add()),

//     for i in 0..SPAWN_COUNT {
//         let mut rng = rand::thread_rng();
//         let ran_num: f32 = rng.gen_range(0.0..1.0);
//         // Distribute colors evenly across the rainbow.
//         let color = Color::hsl(360. * ran_num as f32, 0.95, 0.7);

//         let (x, y) = ulam::get_xy_from_value(i);
//         let _shape = commands.spawn((
//             Mesh2d(shape.clone()),
//             MeshMaterial2d(materials.add(color)),
//             Transform::from_xyz(
//                 // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
//                 x as f32 * SIZE_OF_SQUARE,
//                 y as f32 * SIZE_OF_SQUARE,
//                 0.0,
//             ),
//         ));
//     }
// }

fn setup(mut commands: Commands, texture_atlas_handle_land: Res<SpriteSheetLand>) {
    //commands.spawn(Camera2d);
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: ZOOM_IN_MAX,
            ..OrthographicProjection::default_2d()
        },
    ));
    //meshes.add()),

    for i in 0..SPAWN_COUNT {
        let (x, y) = ulam::get_xy_from_value(i);
        let mut rng = rand::thread_rng();
        let ld0_num: usize = rng.gen_range(0..=34);

        let transform = Transform {
            translation: Vec3 {
                x: x as f32 * SIZE_OF_SQUARE + GRID_SPACING,
                y: y as f32 * SIZE_OF_SQUARE + GRID_SPACING,
                z: 0.0,
            },
            scale: Vec3::new(SCALE_FACTOR, SCALE_FACTOR, 1.0),
            ..Default::default()
        };

        commands.spawn((
            Sprite {
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_handle_land.layout.clone(),
                    index: ld0_num,
                }),
                image: texture_atlas_handle_land.texture.clone(),
                ..Default::default()
            },
            transform,
            LevelOfDetail::Ld0,
        ));
    }
    for i in 0..SPAWN_COUNT / 4 {
        let (x, y) = ulam::get_xy_from_value(i);
        let mut rng = rand::thread_rng();
        let ld1_num: usize = rng.gen_range(0..=4);

        let transform = Transform {
            translation: Vec3 {
                x: x as f32 * SIZE_OF_SQUARE * 2. + (GRID_SPACING * 2.),
                y: y as f32 * SIZE_OF_SQUARE * 2. + (GRID_SPACING * 2.),
                z: 0.0,
            },
            scale: Vec3::new(SCALE_FACTOR * 2., SCALE_FACTOR * 2., 1.0),
            ..Default::default()
        };

        commands.spawn((
            Sprite {
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_handle_land.layout.clone(),
                    index: ld1_num,
                }),
                image: texture_atlas_handle_land.texture.clone(),
                ..Default::default()
            },
            transform,
            Visibility::Hidden,
            LevelOfDetail::Ld1,
        ));
    }
    for i in 0..SPAWN_COUNT / 16 {
        let (x, y) = ulam::get_xy_from_value(i);
        let mut rng = rand::thread_rng();
        let ld1_num: usize = rng.gen_range(0..=3);

        let transform = Transform {
            translation: Vec3 {
                x: x as f32 * SIZE_OF_SQUARE * 4. + (GRID_SPACING * 4.),
                y: y as f32 * SIZE_OF_SQUARE * 4. + (GRID_SPACING * 4.),
                z: 0.0,
            },
            scale: Vec3::new(SCALE_FACTOR * 4., SCALE_FACTOR * 4., 1.0),
            ..Default::default()
        };

        commands.spawn((
            Sprite {
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_handle_land.layout.clone(),
                    index: ld1_num,
                }),
                image: texture_atlas_handle_land.texture.clone(),
                ..Default::default()
            },
            transform,
            Visibility::Hidden,
            LevelOfDetail::Ld2,
        ));
    }
}

pub fn keyboard_movement_camera_system(
    // mut mouse_motion_events: EventReader<MouseMotion>,
    // mouse: Res<ButtonInput<MouseButton>>,
    mut q_camera: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.pressed(KeyCode::KeyW)
        || keys.pressed(KeyCode::KeyA)
        || keys.pressed(KeyCode::KeyS)
        || keys.pressed(KeyCode::KeyD)
        || keys.pressed(KeyCode::ArrowUp)
        || keys.pressed(KeyCode::ArrowLeft)
        || keys.pressed(KeyCode::ArrowDown)
        || keys.pressed(KeyCode::ArrowRight)
    {
        for (mut cam_transform, cam_ortho) in q_camera.iter_mut() {
            let mut direction = Vec3::ZERO;

            if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
                direction.y += 1.0
            }
            if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
                direction.y -= 1.0
            }
            if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
                direction.x -= 1.0
            }
            if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
                direction.x += 1.0
            }

            let move_delta =
                direction.normalize_or_zero() * cam_ortho.scale * MOVE_SPEED * time.delta_secs();
            let clamped_length = move_delta.clamp_length_max(300.0);

            // let clamped_length = total_distance.clamp_length_max(300.0);

            cam_transform.translation += clamped_length;
        }
    }
}

pub fn zoom_wheel_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    time: Res<Time>,
    mut cam_query: Query<&mut OrthographicProjection, With<Camera>>,
    mut event_zoom: EventWriter<LdChange>,
    mut level_of_detail: ResMut<CurrentLd>,
) {
    for mouse_wheel in mouse_wheel_events.read() {
        let zoom_amount = ZOOM_SPEED * time.delta_secs() * mouse_wheel.y;
        for mut ortho in cam_query.iter_mut() {
            let previous_ld = level_of_detail.0;
            ortho.scale -= zoom_amount;
            ortho.scale = ortho.scale.clamp(ZOOM_IN_MAX, ZOOM_OUT_MAX);
            let current_ld = LevelOfDetail::get_level(ortho.scale);
            if current_ld != previous_ld {
                *level_of_detail = CurrentLd(current_ld);
                event_zoom.send(LdChange(current_ld));
            }
        }
    }
}

pub fn visi_tiles(
    mut event: EventReader<LdChange>,
    mut query: Query<(&mut Visibility, &LevelOfDetail)>,
) {
    for e in event.read() {
        match e.0 {
            LevelOfDetail::Ld0 => {
                for (mut v, ld) in query.iter_mut() {
                    match ld {
                        LevelOfDetail::Ld0 => {
                            *v = Visibility::Visible;
                        }
                        _ => {
                            *v = Visibility::Hidden;
                        }
                    }
                }
            }
            LevelOfDetail::Ld1 => {
                for (mut v, ld) in query.iter_mut() {
                    match ld {
                        LevelOfDetail::Ld1 => {
                            *v = Visibility::Visible;
                        }
                        _ => {
                            *v = Visibility::Hidden;
                        }
                    }
                }
            }
            LevelOfDetail::Ld2 => {
                for (mut v, ld) in query.iter_mut() {
                    match ld {
                        LevelOfDetail::Ld2 => {
                            *v = Visibility::Visible;
                        }
                        _ => {
                            *v = Visibility::Hidden;
                        }
                    }
                }
            }
        }
    }
}
