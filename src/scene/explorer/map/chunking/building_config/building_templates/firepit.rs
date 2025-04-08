use bevy::prelude::*;

use crate::scene::initer::ecs::component::{AnimationIndicesComp, AnimationTimerComp};

#[allow(clippy::too_many_arguments)]
pub fn spawn_firepit(
    texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    builder: &mut ChildSpawnerCommands,
    translation: Vec3,
    scale_multiplier: f32,
) {
    let animation_indices = AnimationIndicesComp { first: 9, last: 11 };
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
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layout.clone(),
                index: animation_indices.first,
            }),
            ..Default::default()
        },
        transform,
        AnimationTimerComp(Timer::from_seconds(0.1, TimerMode::Repeating)),
        //BuildingStructure::FirePit,
        // locationcoord,
        animation_indices,
    ));
}
