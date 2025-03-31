use bevy::prelude::*;
use rand::Rng;

use crate::scene::explorer::ecs::hard::{BUILDING_Z, TILE_SIZE};

use super::building_templates::camp::spawn_camp;

#[allow(clippy::too_many_arguments)]
pub fn spawn(
    texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    builder: &mut ChildBuilder,
    color: Color,
) {
    let place_val = TILE_SIZE.x / 2.3 - 10.;
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen_range(-place_val..place_val);
    let y: f32 = rng.gen_range(-place_val..place_val);

    spawn_camp(
        texture,
        layout,
        builder,
        color,
        Vec3::new(x, y, BUILDING_Z),
        0.75,
    );
}
