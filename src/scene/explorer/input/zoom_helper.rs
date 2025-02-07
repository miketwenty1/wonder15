use bevy::prelude::*;

use crate::scene::explorer::ecs::{
    hard::{CLOSE_ZOOM_THRESHOLD, MEDIUM_ZOOM_THRESHOLD},
    resource::ZoomLevelRes,
    state::ExplorerRunningZoomSub2State,
};

pub fn changed_ortho(
    mut zoom_res: ResMut<ZoomLevelRes>,
    cam_query: Query<&OrthographicProjection, (With<Camera>, Changed<OrthographicProjection>)>,
    mut zoom_state: ResMut<NextState<ExplorerRunningZoomSub2State>>,
) {
    for cam in cam_query.iter() {
        let zoom_level = if cam.scale > MEDIUM_ZOOM_THRESHOLD {
            ExplorerRunningZoomSub2State::Far
        } else if cam.scale > CLOSE_ZOOM_THRESHOLD {
            ExplorerRunningZoomSub2State::Medium
        } else {
            ExplorerRunningZoomSub2State::Close
        };

        if zoom_level != zoom_res.0 {
            zoom_res.0 = zoom_level;
            zoom_state.set(zoom_level);
        }
    }
}
