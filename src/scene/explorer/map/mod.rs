use bevy::prelude::*;
use bevy_ecs_tilemap::map::TilemapTileSize;
use resource::{AdditionalSetupTilesTimer, ChunkManager, DespawnRange, TotalTilesSpawned};
use setup::{startup, startup_tilemap};

mod component;
pub mod resource;
pub mod setup;
// mod text2d_chunking;

use crate::scene::ExplorerSubState;

pub const CHUNK_SIZE: UVec2 = UVec2 { x: 4, y: 4 };
// pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
//     x: CHUNK_SIZE.x * 2,
//     y: CHUNK_SIZE.y * 2,
// };

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum InitSpawnTileMap {
    #[default]
    Running,
    Done,
    Off,
}

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 66.0, y: 66.0 };

pub struct ExplorerMapPlugin;

impl Plugin for ExplorerMapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AdditionalSetupTilesTimer(Timer::from_seconds(
            0.05,
            TimerMode::Repeating,
        )))
        .insert_resource(ChunkManager::default())
        .insert_resource(TotalTilesSpawned(0))
        .insert_resource(DespawnRange(CHUNK_SIZE.x as f32 * TILE_SIZE.x * 6.0))
        .init_state::<InitSpawnTileMap>()
        // .add_systems(
        //     OnEnter(InitSpawnTileMap::Running),
        //     (startup).run_if(run_once),
        // )
        .add_systems(
            Update,
            (startup_tilemap).run_if(in_state(InitSpawnTileMap::Running)),
        );
    }
}

// .insert_resource(ChunkManager::default())

// .init_state::<InitState>()
// .add_plugins(TilemapPlugin)
// .add_systems(Startup, (fit_canvas_to_parent, startup, ).chain())
//
//

//
//
// .add_systems(
//     Update,
//     (startup_tilemap).run_if(in_state(InitState::LoadTiles)),
// )
// .add_systems(Update, (helpers::camera::movement, zoom_wheel_system))
// .add_systems(Update, (spawn_chunks_around_camera, despawn_outofrange_chunks))

// use bevy::prelude::*;

// use crate::global::ecs_items::SceneState;

// mod camera;
// mod canvas;
// mod startup;

// pub struct GameInitPlugin;

// impl Plugin for GameInitPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(
//             OnEnter(SceneState::Init),
//             (
//                 fit_canvas_to_parent,
//                 setup_camera,
//                 setup_animation,
//                 // init_hardcoded_res,
//                 // setup_spritesheets,
//                 // init_js_comms_channels,
//             )
//                 .run_if(run_once),
//         )
//         .add_systems(Update, (animate_sprite, swap_tile_index))
//         .insert_resource(TotalTilesSpawned(0))
//         .insert_resource(BonusSpawnTimer(Timer::from_seconds(
//             0.05,
//             TimerMode::Repeating,
//         )))
//         .insert_resource(ChunkManager::default())
//         .insert_resource(DespawnRange(CHUNK_SIZE.x as f32 * TILE_SIZE.x * 6.0));
//     }
// }

// mod canvas;
// mod helpers;
// mod text2d_chunking;
//const TEXT_ZOOM_THRESHOLD: f32 = 2.5;

// // pub const SCALE_FACTOR: f32 = 2.0;
// #[derive(Component, Debug)]
// pub struct YoMap;

// #[derive(Resource, Clone)]
// pub struct SpriteSheetBuilding {
//     pub layout: Handle<TextureAtlasLayout>,
//     pub texture: Handle<Image>,
// }

// // #[derive(Bundle)]
// // pub struct MyTileBundle {
// //     pub tile_bundle: TileBundle,
// //     custom_extras: TileData,
// // }

// // #[derive(Resource, Debug)]
// // pub struct DespawnRange(f32);

// // #[derive(Event, Debug)]
// // pub enum TextVisibilityEvent {
// //     KeyPressToggle,
// //     ButtonToggle,
// //     Zoom,
// // }

// #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
// pub enum InitState {
//     #[default]
//     Off,
//     LoadTiles,
//     Done,
// }
