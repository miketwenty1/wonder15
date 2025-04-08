use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::scene::explorer::ecs::hard::BUILDING_Z;

use super::building_templates::{castle::spawn_castle, road::spawn_road};

#[allow(clippy::too_many_arguments)]
pub fn spawn(
    texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    builder: &mut ChildSpawnerCommands,
    color: Color,
    height: u32,
) {
    spawn_castle(
        texture,
        layout,
        builder,
        color,
        Vec3::new(0., 0., BUILDING_Z),
        2.5,
    );

    spawn_road(texture, layout, builder, WHITE.into(), 5, height);
}
