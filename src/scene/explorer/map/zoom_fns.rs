use crate::scene::{explorer::event::ZoomLevelEvent, ExplorerRunningZoomSub2State};
use bevy::prelude::*;

pub fn zoom_reader(
    mut event: EventReader<ZoomLevelEvent>,
    mut zoom_state: ResMut<NextState<ExplorerRunningZoomSub2State>>,
) {
    for e in event.read() {
        //let zoom = cam_q.get_single().unwrap().scale;

        match e {
            ZoomLevelEvent::Far => {
                zoom_state.set(ExplorerRunningZoomSub2State::Far);
            }
            ZoomLevelEvent::Medium => {
                zoom_state.set(ExplorerRunningZoomSub2State::Medium);
            }
            ZoomLevelEvent::Close => {
                zoom_state.set(ExplorerRunningZoomSub2State::Close);
            }
        }
    }
}
