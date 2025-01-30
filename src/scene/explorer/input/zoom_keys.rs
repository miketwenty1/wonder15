use bevy::{input::ButtonInput, prelude::*, render::camera::Camera};

use crate::scene::explorer::{
    event::ZoomLevelEvent,
    hard::{MAX_ZOOMIN_THRESHOLD, MAX_ZOOMOUT_THRESHOLD},
    resource::ZoomLevelRes,
};

use super::zoom_helper::zoom_event_writer;
pub fn zoom_keyboard(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cam_query: Query<&mut OrthographicProjection, With<Camera>>,
    event: EventWriter<ZoomLevelEvent>,
    zoom_res: ResMut<ZoomLevelRes>,
) {
    let mut pre = 0.;
    let mut post = 0.;
    for mut ortho in cam_query.iter_mut() {
        pre = ortho.scale;
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
        post = ortho.scale;
        //text
    }
    if pre != post {
        zoom_event_writer(post, event, zoom_res);
    }
}
