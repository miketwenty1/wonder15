use bevy::prelude::*;

use super::ecs::resource::ApiPollingTimer;

pub fn tick_api_receive_timer(mut api_timer: ResMut<ApiPollingTimer>, time: Res<Time>) {
    api_timer.timer.tick(time.delta());
}
