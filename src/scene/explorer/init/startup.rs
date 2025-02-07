use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::scene::explorer::ecs::{
    component::{AnimationIndicesComp, AnimationTimerComp},
    hard::ANIMATED_SPRITE_Z,
    resource::SpriteSheetBuildingRes,
    state::{ExplorerSubState, InitSpawnTileMapState},
};

pub fn setup_animation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("we got to explorer init setup");
    let texture = asset_server.load("spritesheet/gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndicesComp { first: 1, last: 6 };
    commands.spawn((
        Sprite {
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            }),
            color: Color::Srgba(Color::BLACK.into()),
            image: texture,
            ..default()
        },
        Transform::from_translation(Vec3 {
            x: 20.0,
            y: 20.0,
            z: ANIMATED_SPRITE_Z,
        }),
        animation_indices,
        AnimationTimerComp(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

pub fn init_startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut init_map_state: ResMut<NextState<InitSpawnTileMapState>>,
    mut explorer_sub_state: ResMut<NextState<ExplorerSubState>>,
) {
    let building_atlas = TextureAtlasLayout::from_grid(
        bevy::prelude::UVec2::new(32, 32),
        18,
        1,
        Some(bevy::prelude::UVec2::new(2, 2)),
        Some(bevy::prelude::UVec2::new(1, 1)),
    );
    let building_texture_atlas = texture_atlases.add(building_atlas);

    let building_texture = asset_server.load_with_settings(
        "spritesheet/buildings1v2.png",
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        },
    );

    commands.insert_resource(SpriteSheetBuildingRes {
        layout: building_texture_atlas,
        texture: building_texture,
    });

    init_map_state.set(InitSpawnTileMapState::Running);
    explorer_sub_state.set(ExplorerSubState::Running);
}
