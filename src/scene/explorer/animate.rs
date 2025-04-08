use bevy::prelude::*;
use rand::seq::IteratorRandom;

use crate::scene::initer::ecs::component::{AnimationIndicesComp, AnimationTimerComp};

use super::ecs::component::{Castle, RunningHal};

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<
        (&AnimationIndicesComp, &mut AnimationTimerComp, &mut Sprite),
        With<RunningHal>,
    >,
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

#[derive(Default)]
pub struct HalMoveState {
    target: Option<Vec3>,
    speed: f32,
}

pub fn random_hal_to_castle(
    time: Res<Time>,
    mut hal_query: Query<&mut Transform, With<RunningHal>>,
    castle_query: Query<&Transform, (With<Castle>, Without<RunningHal>)>,
    mut state: Local<HalMoveState>,
) {
    let dt = time.delta_secs();

    // Get the single Hal transform, if it exists.
    if let Ok(mut hal_transform) = hal_query.single_mut() {
        // If there isn't a target, pick a random castle
        if state.target.is_none() {
            if let Some(castle_transform) = castle_query.iter().choose(&mut rand::thread_rng()) {
                state.target = Some(castle_transform.translation);
                state.speed = 100.0; // or whatever speed you want
            }
        }

        // If we have a target, move toward it
        if let Some(target_pos) = state.target {
            let direction = (target_pos - hal_transform.translation).normalize_or_zero();
            hal_transform.translation += direction * state.speed * dt;

            // If we're close enough to the castle, clear the target so we can pick another next time
            if hal_transform.translation.distance(target_pos) < 10.0 {
                state.target = None;
            }
        }
    }
}
