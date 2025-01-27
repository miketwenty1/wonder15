use bevy::{input::mouse::MouseWheel, prelude::*, time::Time};

pub fn zoom_wheel_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    time: Res<Time>,
    mut cam_query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for mouse_wheel in mouse_wheel_events.read() {
        let zoom_amount = 1.0 * time.delta_secs() * mouse_wheel.y;
        for mut ortho in cam_query.iter_mut() {
            ortho.scale -= zoom_amount;
        }
    }
}
