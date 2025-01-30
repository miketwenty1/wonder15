use bevy::{input::mouse::MouseWheel, prelude::*, time::Time};

use crate::scene::explorer::{
    event::ZoomLevelEvent,
    hard::{MAX_ZOOMIN_THRESHOLD, MAX_ZOOMOUT_THRESHOLD},
    resource::ZoomLevelRes,
};

use super::zoom_helper::zoom_event_writer;

pub fn zoom_wheel_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    time: Res<Time>,
    mut cam_query: Query<&mut OrthographicProjection, With<Camera>>,
    event: EventWriter<ZoomLevelEvent>,
    zoom_res: ResMut<ZoomLevelRes>,
) {
    let mut pre = 0.;
    let mut post = 0.;
    for mouse_wheel in mouse_wheel_events.read() {
        let zoom_amount = 1.0 * time.delta_secs() * mouse_wheel.y;
        for mut ortho in cam_query.iter_mut() {
            pre = ortho.scale;
            if ortho.scale - zoom_amount > MAX_ZOOMOUT_THRESHOLD {
                ortho.scale = MAX_ZOOMOUT_THRESHOLD;
            } else if ortho.scale - zoom_amount < MAX_ZOOMIN_THRESHOLD {
                ortho.scale = MAX_ZOOMIN_THRESHOLD
            } else {
                ortho.scale -= zoom_amount
            }
            post = ortho.scale;
        }
    }
    if pre != post {
        zoom_event_writer(post, event, zoom_res);
    }
}
