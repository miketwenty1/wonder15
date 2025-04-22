use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{
    ecs::resource::WorldOwnedTileMap,
    helper::utils::funs::ulam_to_real_world_xy,
    scene::explorer::{
        ecs::{
            component::{HalSpeed, HalTargetBlock, HalTargetXY, HalThere, HomeTile},
            event::SpawnRunnerMan,
            hard::{
                ANIMATED_SPRITE_Z, BUILDING_CHUNK_SIZE, BUILDING_DESPAWN_RANGE_MULTIPLIER,
                SLIM_BUILDING_CHUNK_SIZE, SLIM_CLOSE_ZOOM_THRESHOLD, SLIM_MAX_ZOOM,
                SLIM_MEDIUM_ZOOM_THRESHOLD, SLIM_TEXT_CHUNK_SIZE, SLIM_TILE_CHUNK_SIZE,
                TEXT_CHUNK_SIZE, TEXT_DESPAWN_RANGE_MULTIPLIER, TILE_CHUNK_SIZE,
                TILE_DESPAWN_RANGE_MULTIPLIER, TILE_SIZE,
            },
            resource::{
                ChunkTypeNumsRes, DespawnBuildingRangeRes, DespawnTextRangeRes,
                DespawnTileRangeRes, SpriteSheetBuildingRes, ZoomLevelNumsRes,
            },
            state::{ExplorerSubState, InitSpawnMapState},
        },
        input::hard::LastClickedTile,
    },
};
use crate::{
    ecs::state::FullMapState,
    scene::{
        explorer::ecs::{
            component::RunningHal,
            hard::{CLOSE_ZOOM_THRESHOLD, MAX_ZOOM, MEDIUM_ZOOM_THRESHOLD, MIN_ZOOM},
            resource::SpriteSheetManualSelectRes,
        },
        initer::ecs::component::{AnimationIndicesComp, AnimationTimerComp},
    },
};

pub fn spawn_running_hal(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut spawn_man: EventReader<SpawnRunnerMan>,
    mut man_count: Local<u32>,
    selected_tile: Res<LastClickedTile>,
    world_owned_map: Res<WorldOwnedTileMap>,
) {
    for _e in spawn_man.read() {
        if let Some(spawn_tile_location) = world_owned_map.map.get(&selected_tile.ulam) {
            *man_count += 1;
            info!("spawn man count: {}", *man_count);
            let texture = asset_server.load("spritesheet/gabe-idle-run.png");
            let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);
            // Use only the subset of sprites in the sheet that make up the run animation
            let animation_indices = AnimationIndicesComp { first: 1, last: 6 };
            let offset = Vec2 {
                x: TILE_SIZE.x / 2.,
                y: TILE_SIZE.y / 2.,
            };
            let real_location =
                ulam_to_real_world_xy(spawn_tile_location.height, TILE_SIZE.x, offset);
            let value = spawn_tile_location.value as f32;
            let scale = (value / 64000.0).clamp(1.0, 2.0);
            let custom_size = Some(Vec2::splat(32.0 * scale));
            commands.spawn((
                Sprite {
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layout,
                        index: animation_indices.first,
                    }),
                    custom_size,
                    color: spawn_tile_location.color, //Color::Srgba(Color::WHITE.into()),
                    image: texture,
                    ..default()
                },
                Transform::from_translation(Vec3 {
                    x: real_location.x,
                    y: real_location.y,
                    z: ANIMATED_SPRITE_Z,
                }),
                animation_indices,
                AnimationTimerComp(Timer::from_seconds(0.1, TimerMode::Repeating)),
                RunningHal,
                HalTargetBlock(0),
                HalTargetXY(Vec2::new(0., 0.)),
                HalSpeed(50.),
                HalThere(false),
                HomeTile(spawn_tile_location.height),
            ));
        }
    }
}

pub fn init_startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut init_map_state: ResMut<NextState<InitSpawnMapState>>,
    mut explorer_sub_state: ResMut<NextState<ExplorerSubState>>,
    full_map_state: Res<State<FullMapState>>,
) {
    let building_atlas = TextureAtlasLayout::from_grid(
        bevy::prelude::UVec2::new(32, 32),
        18,
        1,
        Some(bevy::prelude::UVec2::new(2, 2)),
        Some(bevy::prelude::UVec2::new(1, 1)),
    );
    let building_texture_atlas = texture_atlases.add(building_atlas);

    let building_texture = asset_server.load_with_settings(
        "spritesheet/buildings1v2.png",
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        },
    );

    commands.insert_resource(SpriteSheetBuildingRes {
        layout: building_texture_atlas,
        texture: building_texture,
    });

    let manual_select_atlas = TextureAtlasLayout::from_grid(
        bevy::prelude::UVec2::new(32, 32),
        18,
        1,
        Some(bevy::prelude::UVec2::new(2, 2)),
        Some(bevy::prelude::UVec2::new(1, 1)),
    );
    let manual_select_texture_atlas = texture_atlases.add(manual_select_atlas);

    let manual_select_texture = asset_server.load_with_settings(
        "spritesheet/select.png",
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        },
    );

    commands.insert_resource(SpriteSheetManualSelectRes {
        layout: manual_select_texture_atlas,
        texture: manual_select_texture,
    });

    if *full_map_state == FullMapState::On {
        commands.insert_resource(ZoomLevelNumsRes {
            max_zoom: MAX_ZOOM,
            min_zoom: MIN_ZOOM,
            close_threshold: CLOSE_ZOOM_THRESHOLD,
            medium_threshold: MEDIUM_ZOOM_THRESHOLD,
        });
        commands.insert_resource(ChunkTypeNumsRes {
            building: BUILDING_CHUNK_SIZE,
            tile: TILE_CHUNK_SIZE,
            text: TEXT_CHUNK_SIZE,
        });
        // commands.insert_resource(DespawnTileRangeRes(
        //     TILE_CHUNK_SIZE.x as f32 * TILE_SIZE.x * TILE_DESPAWN_RANGE_MULTIPLIER,
        // ));
        commands.insert_resource(DespawnTextRangeRes(
            TEXT_CHUNK_SIZE.x as f32 * TILE_SIZE.x * TEXT_DESPAWN_RANGE_MULTIPLIER,
        ));
        commands.insert_resource(DespawnBuildingRangeRes(
            BUILDING_CHUNK_SIZE.x as f32 * TILE_SIZE.x * BUILDING_DESPAWN_RANGE_MULTIPLIER,
        ));
    } else {
        commands.insert_resource(ZoomLevelNumsRes {
            max_zoom: SLIM_MAX_ZOOM,
            min_zoom: MIN_ZOOM,
            close_threshold: SLIM_CLOSE_ZOOM_THRESHOLD,
            medium_threshold: SLIM_MEDIUM_ZOOM_THRESHOLD,
        });
        commands.insert_resource(ChunkTypeNumsRes {
            building: SLIM_BUILDING_CHUNK_SIZE,
            tile: SLIM_TILE_CHUNK_SIZE,
            text: SLIM_TEXT_CHUNK_SIZE,
        });
        commands.insert_resource(DespawnTileRangeRes(
            SLIM_TILE_CHUNK_SIZE.x as f32 * TILE_SIZE.x * TILE_DESPAWN_RANGE_MULTIPLIER,
        ));
        commands.insert_resource(DespawnTextRangeRes(
            SLIM_TEXT_CHUNK_SIZE.x as f32 * TILE_SIZE.x * TEXT_DESPAWN_RANGE_MULTIPLIER,
        ));
        commands.insert_resource(DespawnBuildingRangeRes(
            SLIM_BUILDING_CHUNK_SIZE.x as f32 * TILE_SIZE.x * BUILDING_DESPAWN_RANGE_MULTIPLIER,
        ));
    }

    init_map_state.set(InitSpawnMapState::MapSpawn);
    explorer_sub_state.set(ExplorerSubState::Running);
}
