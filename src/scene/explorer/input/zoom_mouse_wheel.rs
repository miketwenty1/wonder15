use bevy::{input::mouse::MouseWheel, prelude::*, time::Time};

use crate::scene::explorer::ecs::resource::ZoomLevelNumsRes;

pub fn zoom_wheel_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    time: Res<Time>,
    cam: Single<&mut Projection, With<Camera>>,
    zooms: Res<ZoomLevelNumsRes>,
) {
    let mut binding = cam.into_inner();
    let ortho = if let Projection::Orthographic(ref mut ortho) = *binding {
        ortho
    } else {
        panic!("no ortho!");
    };

    for mouse_wheel in mouse_wheel_events.read() {
        let zoom_amount = 1.0 * time.delta_secs() * mouse_wheel.y;
        if ortho.scale - zoom_amount > zooms.max_zoom {
            ortho.scale = zooms.max_zoom;
        } else if ortho.scale - zoom_amount < zooms.min_zoom {
            ortho.scale = zooms.min_zoom
        } else {
            ortho.scale -= zoom_amount
        }
    }
}
