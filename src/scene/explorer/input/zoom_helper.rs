use bevy::prelude::*;

use crate::scene::explorer::ecs::{
    resource::{CurrentZoomLevelRes, ZoomLevelNumsRes},
    state::ExplorerRunningZoomSub2State,
};

pub fn changed_ortho(
    mut zoom_res: ResMut<CurrentZoomLevelRes>,
    cam_query: Query<&Projection, (With<Camera>, Changed<Projection>)>,
    mut zoom_state: ResMut<NextState<ExplorerRunningZoomSub2State>>,
    zooms: Res<ZoomLevelNumsRes>,
) {
    for cam_projection in cam_query.iter() {
        let cam_ortho = match *cam_projection {
            Projection::Orthographic(ref ortho) => ortho,
            _ => panic!("Expected Orthographic projection"),
        };

        let zoom_level = if cam_ortho.scale > zooms.medium_threshold {
            ExplorerRunningZoomSub2State::Far
        } else if cam_ortho.scale > zooms.close_threshold {
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
