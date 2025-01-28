use bevy::{input::ButtonInput, math::Vec3, prelude::*, render::camera::Camera};

use crate::scene::explorer::hard::{MAX_ZOOMIN_THRESHOLD, MAX_ZOOMOUT_THRESHOLD};
pub fn keyboard_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cam_query: Query<&mut OrthographicProjection, With<Camera>>,
    // mut despawn_range: ResMut<DespawnRange>,
    // mut text_visi_event: EventWriter<TextVisibilityEvent>,
) {
    for mut ortho in cam_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyZ) {
            if ortho.scale + 0.2 > MAX_ZOOMOUT_THRESHOLD {
                ortho.scale = MAX_ZOOMOUT_THRESHOLD;
            } else {
                ortho.scale += 0.2;
            }
        }

        if keyboard_input.pressed(KeyCode::KeyX) {
            if ortho.scale - 0.2 < MAX_ZOOMIN_THRESHOLD {
                ortho.scale = MAX_ZOOMIN_THRESHOLD;
            } else {
                ortho.scale -= 0.2;
            }
        }
    }
}
