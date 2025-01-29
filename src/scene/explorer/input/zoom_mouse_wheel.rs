use bevy::{input::mouse::MouseWheel, prelude::*, time::Time};

use crate::scene::explorer::{
    event::TextVisibilityEvent,
    hard::{
        BUILDING_VISIBILITY_ZOOM_THRESHOLD, MAX_ZOOMIN_THRESHOLD, MAX_ZOOMOUT_THRESHOLD,
        TEXT_VISIBILITY_ZOOM_THRESHOLD,
    },
};

pub fn zoom_wheel_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    time: Res<Time>,
    mut cam_query: Query<&mut OrthographicProjection, With<Camera>>,
    mut text_visi_event: EventWriter<TextVisibilityEvent>,
) {
    for mouse_wheel in mouse_wheel_events.read() {
        let zoom_amount = 1.0 * time.delta_secs() * mouse_wheel.y;
        for mut ortho in cam_query.iter_mut() {
            let pre = ortho.scale;
            if ortho.scale - zoom_amount > MAX_ZOOMOUT_THRESHOLD {
                ortho.scale = MAX_ZOOMOUT_THRESHOLD;
            } else if ortho.scale - zoom_amount < MAX_ZOOMIN_THRESHOLD {
                ortho.scale = MAX_ZOOMIN_THRESHOLD
            } else {
                ortho.scale -= zoom_amount
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
            if pre < BUILDING_VISIBILITY_ZOOM_THRESHOLD && post > BUILDING_VISIBILITY_ZOOM_THRESHOLD
            {
                text_visi_event.send(TextVisibilityEvent::ZoomOut);
            }
            if pre > BUILDING_VISIBILITY_ZOOM_THRESHOLD && post < BUILDING_VISIBILITY_ZOOM_THRESHOLD
            {
                text_visi_event.send(TextVisibilityEvent::ZoomIn);
            }
        }
    }
}
