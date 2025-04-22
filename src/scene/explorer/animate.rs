use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{seq::IteratorRandom, thread_rng};

use crate::scene::{
    explorer::ecs::hard::ANIMATED_SPRITE_Z,
    initer::ecs::component::{AnimationIndicesComp, AnimationTimerComp},
};

use super::{
    ecs::{
        component::{
            BuildingTileComp, HalPower, HalSpeed, HalTargetBlock, HalTargetXY, HalThere, HomeTile,
            RunningHal,
        },
        hard::{BUILDING_CHUNK_SIZE, TILE_SIZE},
    },
    map::ecs::component::{ChunkBuildingMapComp, RealTileXY, UlamComp},
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

#[allow(clippy::type_complexity)]
pub fn random_hal_walk(
    time: Res<Time>,
    mut hal_query: Query<
        (
            &mut Transform,
            &HalSpeed,
            &mut HalThere,
            &mut HalTargetBlock,
            &mut HalTargetXY,
            &mut Sprite,
        ),
        With<RunningHal>,
    >,
    building_tile_q: Query<(&UlamComp, &RealTileXY), With<BuildingTileComp>>,
    //castle_q: Query<&ChildOf, With<Castle>>,
    // building_map_q: Query<
    //     (
    //         &TilemapType,
    //         &TilemapGridSize,
    //         &TilemapTileSize,
    //         &TileStorage,
    //         &TilemapSize,
    //         &TilemapAnchor,
    //     ),
    //     With<ChunkBuildingMapComp>,
    // >,
) {
    let dt = time.delta_secs();

    for (mut transform, speed, mut there, mut target_block, mut target_xy, mut sprite) in
        hal_query.iter_mut()
    {
        if there.0 {
            // if hal is at his target let's give him a new destination
            let mut rng = thread_rng();
            // let mut all_tiles = building_map_q.iter().flat_map(
            //     |(map_type, grid_size, tile_size, tilemap_storage, map_size, anchor)| {
            //         tilemap_storage.iter().flatten().map(move |&tile_entity| {
            //             (
            //                 tile_entity,
            //                 // *map_type,
            //                 // *grid_size,
            //                 // *tile_size,
            //                 // *map_size,
            //                 // *anchor,
            //             )
            //         })
            //     },
            // );
            // pick a random tile with a buidling and set it for hal

            // // pick a specific tile
            // if let Some((tile_entity, map_type, grid_size, tile_size, map_size, anchor)) = all_tiles
            //     .find(|(tile_entity, _, _, _, _, _)| {
            //         if let Ok((_, ulam, _)) = building_tile_q.get(*tile_entity) {
            //             ulam.0 == 265
            //         } else {
            //             false
            //         }
            //     })

            // if let Some((tile_entity, map_type, grid_size, tile_size, map_size, anchor)) =
            //     building_tile_q.choose(&mut rng)
            // {
            if let Some((ulam, real_xy)) = building_tile_q.iter().choose(&mut rand::thread_rng()) {
                // if let Ok((tp, ulam, real_xy)) = building_tile_q.get(tile_entity) {
                // let tile_center = tp
                //     .center_in_world(&map_size, &grid_size, &tile_size, &map_type, &anchor)
                //     .extend(1.0);
                **target_xy = Vec2 {
                    x: real_xy.0.x,
                    y: real_xy.0.y,
                };
                // info!(
                //     "target set to block {} - xy:{},{}",
                //     ulam.0, real_xy.0.x, real_xy.0.y
                // );
                // }
            }

            // for (map_type, grid_size, tile_size, tilemap_storage, map_size, anchor) in
            //     building_map_q.iter()
            // {
            //     if let Some(&tile_entity) =
            //         tilemap_storage.iter().flatten().choose(&mut thread_rng())
            //     {
            //         let tile_pos = building_tile_q.get(tile_entity);
            //         if let Ok((tp, ulam)) = tile_pos {
            //             let tile_center = tp
            //                 .center_in_world(map_size, grid_size, tile_size, map_type, anchor)
            //                 .extend(1.0);
            //             **target_xy = Vec2 {
            //                 x: tile_center.x,
            //                 y: -tile_center.y + (BUILDING_CHUNK_SIZE.x as f32 * TILE_SIZE.x / 2.),
            //             };
            //             info!("target set to block {}", ulam.0);
            //         }
            //     }
            // }
            there.0 = false;
        }

        if !there.0 {
            let distance_left = transform.translation.xy().distance(**target_xy);
            if distance_left < 20.0 {
                there.0 = true;
                //info!("a hal now unset");
            } else {
                let starting_xy = transform.translation.xy();
                // Fix the direction by normalizing the vector from starting to target
                let direction = (**target_xy - starting_xy).normalize();
                sprite.flip_x = direction.x < 0.;
                let change = direction * speed.0 * dt;
                let new_pos = starting_xy + change;

                // info!(
                //     "starting_xy {} change {} new_pos {} distance_left {} tgt {}",
                //     starting_xy,
                //     change,
                //     new_pos,
                //     distance_left,
                //     target_pos.xy()
                // );

                transform.translation = Vec3::new(new_pos.x, new_pos.y, ANIMATED_SPRITE_Z);
            }
        }
    }
}

const CELL_SIZE: f32 = 10.0;

pub fn detect_fight(
    mut commands: Commands,
    hal_query: Query<(Entity, &HomeTile, &Transform), With<RunningHal>>,
) {
    let mut grid: HashMap<(i32, i32), Vec<(Entity, &HomeTile, Vec2)>> = HashMap::new();

    for (entity, home, transform) in hal_query.iter() {
        let pos = transform.translation.truncate();
        let cell = (
            (pos.x / CELL_SIZE).floor() as i32,
            (pos.y / CELL_SIZE).floor() as i32,
        );
        grid.entry(cell).or_default().push((entity, home, pos));
    }

    for ((cx, cy), entities) in &grid {
        for dx in -1..=1 {
            for dy in -1..=1 {
                let neighbor_cell = (cx + dx, cy + dy);
                if let Some(neighbors) = grid.get(&neighbor_cell) {
                    for (e1, h1, p1) in entities {
                        for (e2, h2, p2) in neighbors {
                            if e1 == e2 || h1.0 == h2.0 {
                                continue;
                            }
                            if p1.distance_squared(*p2) <= 100.0 {
                                if rand::random() {
                                    commands.entity(*e1).despawn();
                                } else {
                                    commands.entity(*e2).despawn();
                                }
                                // Optional: return if only one fight per frame
                            }
                        }
                    }
                }
            }
        }
    }
}
