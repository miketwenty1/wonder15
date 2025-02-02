use super::state::TextToggleState;
use crate::scene::explorer::event::TextToggleEvent;
use bevy::prelude::*;

pub fn text_toggle_reader(
    mut event: EventReader<TextToggleEvent>,
    mut text_state: ResMut<NextState<TextToggleState>>,
    text_state_r: Res<State<TextToggleState>>,
    //cam_q: Query<&OrthographicProjection, With<Camera>>,
) {
    for _e in event.read() {
        {
            let t_state = **text_state_r;
            if t_state == TextToggleState::On {
                text_state.set(TextToggleState::Off);
            } else {
                text_state.set(TextToggleState::On);
            }
        }
    }
}
