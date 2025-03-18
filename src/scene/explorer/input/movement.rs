use bevy::{input::ButtonInput, math::Vec3, prelude::*, render::camera::Camera};

use super::hard::MOVE_VELOCITY;
pub fn keyboard_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Projection), With<Camera>>,
) {
    for (mut transform, cam_ortho_projection) in query.iter_mut() {
        let cam_ortho = match *cam_ortho_projection {
            Projection::Orthographic(ref ortho) => ortho,
            _ => panic!("Expected Orthographic projection"),
        };

        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }
        transform.translation +=
            time.delta_secs() * direction * (MOVE_VELOCITY + 250.) * cam_ortho.scale;
    }
}
