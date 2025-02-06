use bevy::prelude::*;

use crate::{
    ecs::{
        resource::{BlockchainHeight, FullMapLength},
        state::SceneState,
    },
    scene::initer::ecs::component::{AnimationIndicesComp, AnimationTimerComp},
};

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndicesComp, &mut AnimationTimerComp, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

pub fn setup_things(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut scene_state: ResMut<NextState<SceneState>>,
    current_blockheight: Res<BlockchainHeight>,
) {
    let map_side_length = ((current_blockheight.0 as f64).sqrt().ceil()) as u32 + 2;
    commands.insert_resource(FullMapLength(map_side_length));
    let texture = asset_server.load("spritesheet/gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndicesComp { first: 1, last: 6 };
    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        Transform::from_translation(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 6.0,
        }),
        animation_indices,
        AnimationTimerComp(Timer::from_seconds(0.1, TimerMode::Repeating)),
        StateScoped(SceneState::Init),
    ));

    scene_state.set(SceneState::Explorer);
}
