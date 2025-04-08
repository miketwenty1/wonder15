use bevy::{input::mouse::MouseMotion, prelude::*};

use super::hard::MOVE_VELOCITY;

#[allow(clippy::too_many_arguments)]
pub fn mouse_movement_camera_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut cam: Single<(&mut Transform, &mut Projection), With<Camera>>,
    time: Res<Time>,
    // mut clear_last_selected: EventWriter<ClearLastSelectedTile>,
) {
    for event in mouse_motion_events.read() {
        if mouse.pressed(MouseButton::Middle)
            || mouse.pressed(MouseButton::Left)
            || mouse.pressed(MouseButton::Right)
        {
            let ortho = if let Projection::Orthographic(ref mut ortho) = *cam.1 {
                ortho
            } else {
                panic!("no ortho!");
            };

            let timefactor = if time.delta_secs() > 0.01 {
                0.01
            } else {
                time.delta_secs()
            };

            let distance = Vec3::new(-event.delta.x, event.delta.y, 0.)
                * ortho.scale
                * MOVE_VELOCITY
                * timefactor;
            let clamped_distance = distance.clamp_length_max(300.);
            cam.0.translation += clamped_distance;

            // set_camera_tile_bounds(cam_transform.translation, &mut edge, &mut edge_event);
        }
    }
}
