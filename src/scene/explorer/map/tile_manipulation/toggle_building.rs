use crate::scene::explorer::{
    ecs::event::BuildingToggleEvent, map::ecs::state::BuildingToggleState,
};
use bevy::prelude::*;

pub fn building_toggle_reader(
    mut event: EventReader<BuildingToggleEvent>,
    mut building_state: ResMut<NextState<BuildingToggleState>>,
    building_state_r: Res<State<BuildingToggleState>>,
    //cam_q: Query<&OrthographicProjection, With<Camera>>,
) {
    for _e in event.read() {
        {
            let t_state = **building_state_r;
            if t_state == BuildingToggleState::On {
                building_state.set(BuildingToggleState::Off);
            } else {
                building_state.set(BuildingToggleState::On);
            }
        }
    }
}
