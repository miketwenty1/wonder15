use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

#[derive(Resource, Clone)]
pub struct SpriteSheetLand {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

pub fn setup_spritesheets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let land_texture = asset_server.load_with_settings(
        "spritesheet/ss-land-v12.png",
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        },
    );

    let land_atlas = TextureAtlasLayout::from_grid(
        bevy::prelude::UVec2::new(32, 32),
        6,
        7,
        Some(bevy::prelude::UVec2::new(2, 2)),
        Some(bevy::prelude::UVec2::new(1, 1)),
    );
    let land_texture_atlas = texture_atlases.add(land_atlas);

    commands.insert_resource(SpriteSheetLand {
        layout: land_texture_atlas,
        texture: land_texture,
    });
}
