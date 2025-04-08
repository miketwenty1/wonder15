use bevy::prelude::*;

use crate::scene::explorer::ecs::{
    resource::{CurrentZoomLevelRes, ZoomLevelNumsRes},
    state::ExplorerRunningZoomSub2State,
};

pub fn changed_ortho(
    mut zoom_res: ResMut<CurrentZoomLevelRes>,
    cam: Single<&mut Projection, (With<Camera>, Changed<Projection>)>,
    mut zoom_state: ResMut<NextState<ExplorerRunningZoomSub2State>>,
    zooms: Res<ZoomLevelNumsRes>,
) {
    let mut binding = cam.into_inner();
    let ortho = if let Projection::Orthographic(ref mut ortho) = *binding {
        ortho
    } else {
        panic!("no ortho!");
    };

    let zoom_level = if ortho.scale > zooms.medium_threshold {
        ExplorerRunningZoomSub2State::Far
    } else if ortho.scale > zooms.close_threshold {
        ExplorerRunningZoomSub2State::Medium
    } else {
        ExplorerRunningZoomSub2State::Close
    };

    if zoom_level != zoom_res.0 {
        zoom_res.0 = zoom_level;
        zoom_state.set(zoom_level);
    }
}
