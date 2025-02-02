use super::hard::{BUILDING_DESPAWN_RANGE, TEXT_DESPAWN_RANGE};
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
        .insert_resource(TotalTilesSpawnedRes(0))
        .insert_resource(DespawnTextRangeRes(TEXT_DESPAWN_RANGE))
        .insert_resource(DespawnBuildingRangeRes(BUILDING_DESPAWN_RANGE));
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct AdditionalSetupTilesTimerRes(pub Timer);

#[derive(Resource, Clone)]
pub struct SpriteSheetBuildingRes {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
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
pub struct DespawnTextRangeRes(pub f32);

#[derive(Resource, Debug)]
pub struct DespawnBuildingRangeRes(pub f32);

#[derive(Resource, Debug)]
pub struct TotalTilesSpawnedRes(pub u32);

// #[derive(Resource, Debug)]
// pub struct TileTextVisibilityRes {
//     pub given_zoom: Visibility,
//     pub when_possible: Visibility,
// }

// #[derive(Resource, Debug)]
// pub struct TileBuildingVisibilityRes {
//     pub given_zoom: Visibility,
//     pub when_possible: Visibility,
// }

// impl TileTextVisibilityRes {
//     /// Returns the next variant in the iteration sequence.
//     pub fn visi_or_nawh(&self) -> Visibility {
//         if self.given_zoom == Visibility::Visible && self.when_possible == Visibility::Visible {
//             Visibility::Visible
//         } else {
//             Visibility::Hidden
//         }
//     }
// }

// impl TileBuildingVisibilityRes {
//     /// Returns the next variant in the iteration sequence.
//     pub fn visi_or_nawh(&self) -> Visibility {
//         if self.given_zoom == Visibility::Visible && self.when_possible == Visibility::Visible {
//             Visibility::Visible
//         } else {
//             Visibility::Hidden
//         }
//     }
// }

//pub TextToggleEventRes
