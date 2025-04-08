use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn spawn_camp(
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
                index: 1,
            }),
            image: texture.clone(),
            ..default()
        },
        transform,
        //BuildingStructure::Camp,
    ));
}
