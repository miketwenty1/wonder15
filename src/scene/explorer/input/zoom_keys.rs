use bevy::{input::ButtonInput, prelude::*, render::camera::Camera};

use crate::scene::explorer::ecs::resource::ZoomLevelNumsRes;

pub fn zoom_keyboard(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    cam: Single<&mut Projection, With<Camera>>,
    zooms: Res<ZoomLevelNumsRes>,
) {
    let mut binding = cam.into_inner();
    let ortho = if let Projection::Orthographic(ref mut ortho) = *binding {
        ortho
    } else {
        panic!("no ortho!");
    };

    if keyboard_input.pressed(KeyCode::KeyZ) {
        if ortho.scale + 0.2 > zooms.max_zoom {
            ortho.scale = zooms.max_zoom;
        } else {
            ortho.scale += 0.2;
        }
    }

    if keyboard_input.pressed(KeyCode::KeyX) {
        if ortho.scale - 0.2 < zooms.min_zoom {
            ortho.scale = zooms.min_zoom;
        } else {
            ortho.scale -= 0.2;
        }
    }
}
