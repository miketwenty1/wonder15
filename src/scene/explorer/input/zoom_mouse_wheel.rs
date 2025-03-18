use bevy::{input::mouse::MouseWheel, prelude::*, time::Time};

use crate::scene::explorer::ecs::resource::ZoomLevelNumsRes;

pub fn zoom_wheel_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    time: Res<Time>,
    mut cam_query: Query<&mut Projection, With<Camera>>,
    zooms: Res<ZoomLevelNumsRes>,
) {
    for mouse_wheel in mouse_wheel_events.read() {
        let cam = cam_query.single_mut().unwrap().into_inner();
        let cam_ortho = match *cam {
            Projection::Orthographic(ref mut ortho) => ortho,
            _ => panic!("Expected Orthographic projection"),
        };

        let zoom_amount = 1.0 * time.delta_secs() * mouse_wheel.y;
        if cam_ortho.scale - zoom_amount > zooms.max_zoom {
            cam_ortho.scale = zooms.max_zoom;
        } else if cam_ortho.scale - zoom_amount < zooms.min_zoom {
            cam_ortho.scale = zooms.min_zoom
        } else {
            cam_ortho.scale -= zoom_amount
        }
    }
}
