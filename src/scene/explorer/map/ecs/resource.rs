use std::collections::HashSet;

use bevy::prelude::*;

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
        .insert_resource(TotalTilesSpawnedRes(0));
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
pub struct TotalTilesSpawnedRes(pub u32);
