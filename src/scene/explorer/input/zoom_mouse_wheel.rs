use bevy::{input::mouse::MouseWheel, prelude::*, time::Time};

use crate::scene::explorer::ecs::resource::ZoomLevelNumsRes;

pub fn zoom_wheel_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    time: Res<Time>,
    mut cam_query: Query<&mut OrthographicProjection, With<Camera>>,
    zooms: Res<ZoomLevelNumsRes>,
) {
    for mouse_wheel in mouse_wheel_events.read() {
        let zoom_amount = 1.0 * time.delta_secs() * mouse_wheel.y;
        for mut ortho in cam_query.iter_mut() {
            if ortho.scale - zoom_amount > zooms.max_zoom {
                ortho.scale = zooms.max_zoom;
            } else if ortho.scale - zoom_amount < zooms.min_zoom {
                ortho.scale = zooms.min_zoom
            } else {
                ortho.scale -= zoom_amount
            }
        }
    }
}
