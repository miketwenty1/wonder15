use bevy::{color::palettes::css::WHITE, prelude::*};
use rand::Rng;

use crate::scene::explorer::ecs::hard::{BUILDING_Z, TILE_SIZE};

use super::building_templates::{house::spawn_house, road::spawn_road, waterwell::spawn_waterwell};

#[allow(clippy::too_many_arguments)]
pub fn spawn(
    texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    builder: &mut ChildBuilder,
    color: Color,
    height: u32,
) {
    let place_val = TILE_SIZE.x / 2.3 - 10.;
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen_range(-place_val..place_val);
    let y: f32 = rng.gen_range(-place_val..place_val);

    spawn_house(
        texture,
        layout,
        builder,
        color,
        Vec3::new(x, y, BUILDING_Z - (y / 10_000.)),
        1.,
    );

    let x: f32 = rng.gen_range(-place_val..place_val);
    let y: f32 = rng.gen_range(-place_val..place_val);
    spawn_waterwell(
        texture,
        layout,
        builder,
        color,
        Vec3::new(x, y, BUILDING_Z - (y / 10_000.)),
        0.75,
        1,
    );

    spawn_road(texture, layout, builder, WHITE.into(), 5, height);
}
