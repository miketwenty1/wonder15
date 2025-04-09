use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::seq::IteratorRandom;

use crate::scene::initer::ecs::component::{AnimationIndicesComp, AnimationTimerComp};

use super::{
    ecs::component::{Castle, RunningHal},
    map::ecs::component::ChunkBuildingMapComp,
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

#[derive(Default)]
pub struct HalMoveState {
    target: Option<Vec2>,
    speed: f32,
}

#[allow(clippy::type_complexity)]
pub fn random_hal_to_castle(
    time: Res<Time>,
    mut hal_query: Query<&mut Transform, With<RunningHal>>,
    castle_q: Query<&mut TilePos, With<Castle>>,
    tilemap_q: Query<
        (
            &Transform,
            &TilemapType,
            &TilemapGridSize,
            &TilemapTileSize,
            &TileStorage,
            &TilemapSize,
            &TilemapAnchor,
        ),
        (With<ChunkBuildingMapComp>, Without<RunningHal>),
    >,
    mut state: Local<HalMoveState>,
) {
    let dt = time.delta_secs();

    // Get the single Hal transform, if it exists.
    if let Ok(mut hal_transform) = hal_query.single_mut() {
        // If there isn't a target, pick a random castle
        if state.target.is_none() {
            for (
                map_transform,
                map_type,
                grid_size,
                tile_size,
                tilemap_storage,
                map_size,
                anchor,
            ) in tilemap_q.iter()
            {
                if let Some(tile_entity) = tilemap_storage.iter().choose(&mut rand::thread_rng()) {
                    match tile_entity {
                        Some(s) => {
                            let tile_pos = castle_q.get(*s);
                            match tile_pos {
                                Ok(r) => {
                                    let tile_center = r
                                        .center_in_world(
                                            map_size, grid_size, tile_size, map_type, anchor,
                                        )
                                        .extend(1.0);
                                    let castle_transform =
                                        *map_transform * Transform::from_translation(tile_center);

                                    state.target = Some(castle_transform.translation.xy());
                                    state.speed = 100.0; // or whatever speed you want
                                    info!("hal now targeting {:?}", state.target);
                                }
                                Err(_) => {}
                            }
                        }
                        None => {
                            // info!("CASTLE NONE");
                        }
                    }
                }
            }
        }

        // If we have a target, move toward it
        if let Some(target_pos) = state.target {
            let direction = (target_pos - hal_transform.translation.xy()).normalize_or_zero();
            let new_pos = hal_transform.translation.xy() + direction * state.speed * dt;
            hal_transform.translation =
                Vec3::new(new_pos.x, new_pos.y, hal_transform.translation.z);

            info!("hal going in this direction: {}", direction);

            // If we're close enough to the castle, clear the target so we can pick another next time
            if hal_transform.translation.xy().distance(target_pos.xy()) < 10.0 {
                state.target = None;
                info!("hal now unset");
            }
        }
    }
}
