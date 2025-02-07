use bevy::prelude::*;

use crate::scene::explorer::map::ecs::{
    component::{ChunkBuildingMapComp, ChunkTextMapComp},
    resource::{ChunkBuildingManagerRes, ChunkTextManagerRes},
};

pub fn despawn_text(
    mut commands: Commands,
    chunks_query_map: Query<Entity, With<ChunkTextMapComp>>,
    mut chunk_manager: ResMut<ChunkTextManagerRes>,
) {
    chunk_manager.spawned_chunks.clear();
    for entity in chunks_query_map.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn despawn_buildings(
    mut commands: Commands,
    chunks_query_map: Query<Entity, With<ChunkBuildingMapComp>>,
    mut chunk_manager: ResMut<ChunkBuildingManagerRes>,
) {
    chunk_manager.spawned_chunks.clear();
    for entity in chunks_query_map.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
