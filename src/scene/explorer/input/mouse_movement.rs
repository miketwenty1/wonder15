use bevy::{input::mouse::MouseMotion, prelude::*};

use super::hard::MOUSE_MOVE_VELOCITY;

#[allow(clippy::too_many_arguments)]
pub fn mouse_movement_camera_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut q_camera: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
    time: Res<Time>,
    // mut clear_last_selected: EventWriter<ClearLastSelectedTile>,
) {
    for event in mouse_motion_events.read() {
        if mouse.pressed(MouseButton::Middle)
            || mouse.pressed(MouseButton::Left)
            || mouse.pressed(MouseButton::Right)
        {
            for (mut cam_transform, cam_ortho) in q_camera.iter_mut() {
                let distance = Vec3::new(-event.delta.x, event.delta.y, 0.)
                    * cam_ortho.scale
                    * MOUSE_MOVE_VELOCITY;
                let clamped_distance = distance.clamp_length_max(300.);
                cam_transform.translation += clamped_distance;

                // set_camera_tile_bounds(cam_transform.translation, &mut edge, &mut edge_event);
            }
        }
    }
}
