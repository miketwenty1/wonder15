use bevy::prelude::*;

use crate::scene::explorer::ecs::component::Castle;

#[allow(clippy::too_many_arguments)]
pub fn spawn_castle(
    texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    builder: &mut ChildSpawnerCommands,
    color: Color,
    translation: Vec3,
    scale_multiplier: f32,
) {
    let transform = Transform {
        translation,
        scale: Vec3 {
            x: scale_multiplier,
            y: scale_multiplier,
            z: 1.,
        },
        ..Default::default()
    };
    builder.spawn((
        Sprite {
            color,
            texture_atlas: Some(TextureAtlas {
                layout: layout.clone(),
                index: 17,
            }),
            image: texture.clone(),
            ..Default::default()
        },
        transform,
        Castle,
    ));
}
