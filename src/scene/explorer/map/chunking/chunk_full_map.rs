use bevy::prelude::*;
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapId, TilemapType},
    tiles::{TileBundle, TileColor, TilePos, TileStorage, TileTextureIndex},
};

use crate::{
    ecs::{
        resource::{BlockchainHeight, FullMapLength, WorldOwnedTileMap},
        state::ExplorerCommsSubState,
    },
    helper::plugins::comms::ecs::{event::RequestServerGameTiles, structy::TileUpdatePattern},
    scene::explorer::{
        ecs::{component::SelectedTile, hard::TILE_SIZE, state::InitSpawnMapState},
        map::ecs::{
            component::{BaseTile, LandIndexComp, MainBaseTileMap, PlayerTileColorComp, UlamComp},
            hard::{
                CHUNK_INIT_LOAD_SIZE, DEFAULT_UNSET_TILE_INDEX, TEXTURE_INDEX_FOR_PLAYER_COLOR,
            },
            resource::{AdditionalSetupTilesTimerRes, TotalTilesSpawnedRes},
        },
    },
};

#[allow(clippy::too_many_arguments)]
pub fn startup_fullmap(
    mut commands: Commands,
    mut tile_storage_q: Query<(Entity, &mut TileStorage), With<MainBaseTileMap>>,
    mut state: ResMut<NextState<InitSpawnMapState>>,
    mut total_tiles_res: ResMut<TotalTilesSpawnedRes>,
    time: Res<Time>,
    mut timer: ResMut<AdditionalSetupTilesTimerRes>,
    map_length: Res<FullMapLength>,
    current_block_height: Res<BlockchainHeight>,
    world_map: Res<WorldOwnedTileMap>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }
    let uoff = map_length.0 / 2; // 1000 / 2; // 1000 x 1000
    let previous = total_tiles_res.0;
    let new_destionation = previous + CHUNK_INIT_LOAD_SIZE;
    //info!("previous: {}, destination: {}", previous, new_destionation);

    for (tilemap_ent, mut tile_storage) in tile_storage_q.iter_mut() {
        for i in previous..new_destionation {
            if total_tiles_res.0 == current_block_height.0 {
                info!("end");
                state.set(InitSpawnMapState::LocalStorageRead);
                return;
            }
            let tile_from_owned_map = world_map.map.get(&i);

            let (land_index, player_tile_color) = match tile_from_owned_map {
                Some(s) => (s.land_index, s.color),
                None => (DEFAULT_UNSET_TILE_INDEX, Color::Srgba(Color::BLACK.into())),
            };
            let (sx, sy) = ulam::get_xy_from_value(i);
            let tile_pos = TilePos {
                x: (sx + uoff as i32) as u32,
                y: (sy + uoff as i32) as u32,
            };

            //let transform = Transform::from_xyz(0., 0., 0.);
            // doing this to counter act the uoff

            let transform =
                Transform::from_xyz(sx as f32 * TILE_SIZE.x, sy as f32 * TILE_SIZE.y, 50.);
            let tile = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_ent),
                        texture_index: TileTextureIndex(TEXTURE_INDEX_FOR_PLAYER_COLOR),
                        color: TileColor(player_tile_color),
                        ..Default::default()
                    },
                    BaseTile,
                    UlamComp(i),
                    PlayerTileColorComp(TileColor(player_tile_color)),
                    LandIndexComp(land_index),
                    SelectedTile(false),
                    transform,
                ))
                .id();

            tile_storage.set(&tile_pos, tile);
            total_tiles_res.0 += 1;
        }
    }
}
