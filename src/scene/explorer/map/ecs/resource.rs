use super::hard::{BUILDING_DESPAWN_RANGE, TEXT_DESPAWN_RANGE, TILE_DESPAWN_RANGE};
use bevy::{prelude::*, utils::HashSet};

pub struct MapResPlugin;

impl Plugin for MapResPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AdditionalSetupTilesTimerRes(Timer::from_seconds(
            0.05,
            TimerMode::Repeating,
        )))
        .insert_resource(ChunkTextManagerRes::default())
        .insert_resource(ChunkBuildingManagerRes::default())
        .insert_resource(ChunkTileManagerRes::default())
        .insert_resource(TotalTilesSpawnedRes(0))
        .insert_resource(DespawnTileRangeRes(TILE_DESPAWN_RANGE))
        .insert_resource(DespawnTextRangeRes(TEXT_DESPAWN_RANGE))
        .insert_resource(DespawnBuildingRangeRes(BUILDING_DESPAWN_RANGE));
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct AdditionalSetupTilesTimerRes(pub Timer);

#[derive(Default, Debug, Resource)]
pub struct ChunkTileManagerRes {
    pub spawned_chunks: HashSet<IVec2>,
}

#[derive(Default, Debug, Resource)]
pub struct ChunkTextManagerRes {
    pub spawned_chunks: HashSet<IVec2>,
}

#[derive(Default, Debug, Resource)]
pub struct ChunkBuildingManagerRes {
    pub spawned_chunks: HashSet<IVec2>,
}

#[derive(Resource, Debug)]
pub struct DespawnTileRangeRes(pub f32);

#[derive(Resource, Debug)]
pub struct DespawnTextRangeRes(pub f32);

#[derive(Resource, Debug)]
pub struct DespawnBuildingRangeRes(pub f32);

#[derive(Resource, Debug)]
pub struct TotalTilesSpawnedRes(pub u32);
