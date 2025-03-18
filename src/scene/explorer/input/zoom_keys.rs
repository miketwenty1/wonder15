use bevy::{input::ButtonInput, prelude::*, render::camera::Camera};

use crate::scene::explorer::ecs::resource::ZoomLevelNumsRes;

pub fn zoom_keyboard(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cam_query: Query<&mut Projection, With<Camera>>,
    zooms: Res<ZoomLevelNumsRes>,
) {
    for mut ortho_projection in cam_query.iter_mut() {
        let cam_ortho = match *ortho_projection {
            Projection::Orthographic(ref mut ortho) => ortho,
            _ => panic!("Expected Orthographic projection"),
        };

        if keyboard_input.pressed(KeyCode::KeyZ) {
            if cam_ortho.scale + 0.2 > zooms.max_zoom {
                cam_ortho.scale = zooms.max_zoom;
            } else {
                cam_ortho.scale += 0.2;
            }
        }

        if keyboard_input.pressed(KeyCode::KeyX) {
            if cam_ortho.scale - 0.2 < zooms.min_zoom {
                cam_ortho.scale = zooms.min_zoom;
            } else {
                cam_ortho.scale -= 0.2;
            }
        }
    }
}
