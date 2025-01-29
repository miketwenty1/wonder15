use bevy::{input::ButtonInput, prelude::*, render::camera::Camera};

use crate::scene::explorer::{
    event::TextVisibilityEvent,
    hard::{
        BUILDING_VISIBILITY_ZOOM_THRESHOLD, MAX_ZOOMIN_THRESHOLD, MAX_ZOOMOUT_THRESHOLD,
        TEXT_VISIBILITY_ZOOM_THRESHOLD,
    },
};
pub fn zoom_keyboard(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cam_query: Query<&mut OrthographicProjection, With<Camera>>,
    // mut despawn_range: ResMut<DespawnRange>,
    // mut text_visi_event: EventWriter<TextVisibilityEvent>,
    mut text_visi_event: EventWriter<TextVisibilityEvent>,
) {
    for mut ortho in cam_query.iter_mut() {
        let pre = ortho.scale;
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
        let post = ortho.scale;
        //text
        if pre < TEXT_VISIBILITY_ZOOM_THRESHOLD && post > TEXT_VISIBILITY_ZOOM_THRESHOLD {
            text_visi_event.send(TextVisibilityEvent::ZoomOut);
        }
        if pre > TEXT_VISIBILITY_ZOOM_THRESHOLD && post < TEXT_VISIBILITY_ZOOM_THRESHOLD {
            text_visi_event.send(TextVisibilityEvent::ZoomIn);
        }

        //building
        if pre < BUILDING_VISIBILITY_ZOOM_THRESHOLD && post > BUILDING_VISIBILITY_ZOOM_THRESHOLD {
            text_visi_event.send(TextVisibilityEvent::ZoomOut);
        }
        if pre > BUILDING_VISIBILITY_ZOOM_THRESHOLD && post < BUILDING_VISIBILITY_ZOOM_THRESHOLD {
            text_visi_event.send(TextVisibilityEvent::ZoomIn);
        }
    }
}
