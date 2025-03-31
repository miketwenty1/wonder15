use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn spawn_waterwell(
    texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    builder: &mut ChildBuilder,
    color: Color,
    translation: Vec3,
    scale_multiplier: f32,
    offset: usize,
) {
    let transform = Transform {
        translation,
        scale: Vec3::new(scale_multiplier, scale_multiplier, 1.0),
        ..Default::default()
    };
    builder.spawn((
        Sprite {
            color,
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layout.clone(),
                index: 15 + offset,
            }),
            ..Default::default()
        },
        transform,
        //BuildingStructure::Waterwell,
    ));
}
