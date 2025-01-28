use bevy::{prelude::*, utils::HashSet};

#[derive(Resource, Deref, DerefMut)]
pub struct AdditionalSetupTilesTimerRes(pub Timer);

#[derive(Resource, Clone)]
pub struct SpriteSheetBuildingRes {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

#[derive(Default, Debug, Resource)]
pub struct ChunkManagerRes {
    pub spawned_chunks: HashSet<IVec2>,
}

#[derive(Resource, Debug)]
pub struct DespawnRangeRes(pub f32);

#[derive(Resource, Debug)]
pub struct TotalTilesSpawnedRes(pub u32);

//pub TextVisibilityEventRes
