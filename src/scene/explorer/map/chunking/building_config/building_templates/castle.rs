use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn spawn_castle(
    texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    builder: &mut ChildBuilder,
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
        //BuildingStructure::Castle,
    ));
}
