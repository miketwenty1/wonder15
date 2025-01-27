use bevy::{prelude::*, utils::HashSet};

#[derive(Resource, Deref, DerefMut)]
pub struct AdditionalSetupTilesTimer(pub Timer);

#[derive(Resource, Clone)]
pub struct SpriteSheetBuilding {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

#[derive(Default, Debug, Resource)]
pub struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

#[derive(Resource, Debug)]
pub struct DespawnRange(pub f32);

#[derive(Resource, Debug)]
pub struct TotalTilesSpawned(pub u32);
