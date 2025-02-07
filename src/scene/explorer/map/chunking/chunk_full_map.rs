use bevy::prelude::*;
use bevy_ecs_tilemap::{
    map::TilemapId,
    tiles::{TileBundle, TileColor, TilePos, TileStorage, TileTextureIndex},
};
use rand::{thread_rng, Rng};

use crate::{
    ecs::resource::{BlockchainHeight, FullMapLength},
    scene::explorer::{
        ecs::state::InitSpawnTileMapState,
        map::ecs::{
            component::{LandIndexComp, MainBaseTileMap, PlayerTileColorComp, UlamComp},
            hard::CHUNK_INIT_LOAD_SIZE,
            resource::{AdditionalSetupTilesTimerRes, TotalTilesSpawnedRes},
        },
    },
};

#[allow(clippy::too_many_arguments)]
pub fn startup_fullmap(
    mut commands: Commands,
    mut tile_storage_q: Query<(Entity, &mut TileStorage), With<MainBaseTileMap>>,
    mut state: ResMut<NextState<InitSpawnTileMapState>>,
    mut total_tiles_res: ResMut<TotalTilesSpawnedRes>,
    time: Res<Time>,
    mut timer: ResMut<AdditionalSetupTilesTimerRes>,
    map_length: Res<FullMapLength>,
    current_block_height: Res<BlockchainHeight>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }
    let uoff = map_length.0 / 2; // 1000 / 2; // 1000 x 1000
    let previous = total_tiles_res.0;
    let new_destionation = previous + CHUNK_INIT_LOAD_SIZE;
    //info!("previous: {}, destination: {}", previous, new_destionation);
    let mut random = thread_rng();

    for (tilemap_ent, mut tile_storage) in tile_storage_q.iter_mut() {
        for i in previous..new_destionation {
            if total_tiles_res.0 == current_block_height.0 {
                state.set(InitSpawnTileMapState::Done);
                return;
            }
            let land_index = random.gen_range(0..=34) as u32; // land tile texture index

            let (sx, sy) = ulam::get_xy_from_value(i);
            let tile_pos = TilePos {
                x: (sx + uoff as i32) as u32,
                y: (sy + uoff as i32) as u32,
            };
            let random_color = if i == 0 {
                TileColor(Color::Srgba(Color::WHITE.into()))
            } else {
                TileColor(Color::Srgba(Color::BLACK.into()))
            };

            //let transform = Transform::from_xyz(0., 0., 0.);
            let tile = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_ent),
                        texture_index: TileTextureIndex(35),
                        color: random_color,
                        ..Default::default()
                    },
                    UlamComp(i),
                    PlayerTileColorComp(random_color),
                    LandIndexComp(land_index),
                    // transform,
                ))
                .id();

            tile_storage.set(&tile_pos, tile);
            total_tiles_res.0 += 1;
        }
    }
}
