use bevy::{input::ButtonInput, math::Vec3, prelude::*, render::camera::Camera};

//use crate::TextVisibilityEvent;

// A simple camera system for moving and zooming the camera.
#[allow(dead_code)]
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    // mut despawn_range: ResMut<DespawnRange>,
    //mut text_visi_event: EventWriter<TextVisibilityEvent>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        // if keyboard_input.pressed(KeyCode::KeyZ) {
        //     ortho.scale += 0.2;
        //     text_visi_event.send(TextVisibilityEvent::Zoom);
        // }

        // if keyboard_input.pressed(KeyCode::KeyX) {
        //     ortho.scale -= 0.2;
        //     text_visi_event.send(TextVisibilityEvent::Zoom);
        // }

        if ortho.scale < 0.15 {
            ortho.scale = 0.15;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_secs() * direction * 300. * ortho.scale;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
    // if keyboard_input.pressed(KeyCode::KeyQ) {
    //     despawn_range.0 += 100.0;
    // }
    // if keyboard_input.pressed(KeyCode::KeyE) {
    //     despawn_range.0 -= 100.0;
    //     if despawn_range.0 < 3000.0 {
    //         despawn_range.0 = 3000.0;
    //     }
    // }
}
