use super::state::BuildingVisibilityState;
use crate::scene::{explorer::event::BuildingVisibilityEvent, ExplorerRunningSub2State};
use bevy::prelude::*;

pub fn building_visibility_reader(
    mut event: EventReader<BuildingVisibilityEvent>,
    mut zoom_state: ResMut<NextState<ExplorerRunningSub2State>>,
    mut building_state: ResMut<NextState<BuildingVisibilityState>>,
    building_state_r: Res<State<BuildingVisibilityState>>,
    //cam_q: Query<&OrthographicProjection, With<Camera>>,
) {
    for e in event.read() {
        //let zoom = cam_q.get_single().unwrap().scale;

        match e {
            BuildingVisibilityEvent::ZoomOut => {
                zoom_state.set(ExplorerRunningSub2State::ZoomFar);
            }
            BuildingVisibilityEvent::ZoomIn => {
                zoom_state.set(ExplorerRunningSub2State::ZoomMedium);
            }
            _ => {
                let b_state = **building_state_r;
                if b_state == BuildingVisibilityState::On {
                    building_state.set(BuildingVisibilityState::Off);
                } else {
                    building_state.set(BuildingVisibilityState::On);
                }
            }
        }
    }
}

// text_visi.0 = if text_visi.0 == Visibility::Visible {
//     Visibility::Hidden
// } else {
//     Visibility::Visible
// };
// if zoom > TEXT_VISIBILITY_ZOOM_THRESHOLD || text_visi.0 == Visibility::Hidden {
//     for mut text_visi in text_q.iter_mut() {
//         *text_visi = Visibility::Hidden;
//     }
// } else {
