use bevy::prelude::*;

use crate::scene::explorer::{
    event::ZoomLevelEvent,
    hard::{CLOSE_ZOOM_THRESHOLD, MEDIUM_ZOOM_THRESHOLD},
    resource::ZoomLevelRes,
};
pub fn zoom_event_writer(
    post_ortho_scale: f32,
    mut event: EventWriter<ZoomLevelEvent>,
    mut zoom_res: ResMut<ZoomLevelRes>,
) {
    let post_zoom_level = if post_ortho_scale > MEDIUM_ZOOM_THRESHOLD {
        ZoomLevelEvent::Far
    } else if post_ortho_scale > CLOSE_ZOOM_THRESHOLD {
        ZoomLevelEvent::Medium
    } else {
        ZoomLevelEvent::Close
    };

    if post_zoom_level != zoom_res.0 {
        zoom_res.0 = post_zoom_level.clone();
        event.send(post_zoom_level);
    }
}
