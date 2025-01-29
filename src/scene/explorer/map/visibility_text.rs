use super::state::TextVisibilityState;
use crate::scene::{explorer::event::TextVisibilityEvent, ExplorerRunningSub2State};
use bevy::prelude::*;

pub fn text_visibility_reader(
    mut event: EventReader<TextVisibilityEvent>,
    mut zoom_state: ResMut<NextState<ExplorerRunningSub2State>>,
    mut text_state: ResMut<NextState<TextVisibilityState>>,
    text_state_r: Res<State<TextVisibilityState>>,
    //cam_q: Query<&OrthographicProjection, With<Camera>>,
) {
    for e in event.read() {
        //let zoom = cam_q.get_single().unwrap().scale;

        match e {
            TextVisibilityEvent::ZoomOut => {
                zoom_state.set(ExplorerRunningSub2State::ZoomMedium);
            }
            TextVisibilityEvent::ZoomIn => {
                zoom_state.set(ExplorerRunningSub2State::ZoomClose);
            }
            _ => {
                let t_state = **text_state_r;
                if t_state == TextVisibilityState::On {
                    text_state.set(TextVisibilityState::Off);
                } else {
                    text_state.set(TextVisibilityState::On);
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
