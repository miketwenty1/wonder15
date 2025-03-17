use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{
    ecs::state::FullMapState,
    helper::plugins::comms::ecs::structy::GetTileType,
    scene::explorer::ecs::hard::{CLOSE_ZOOM_THRESHOLD, MAX_ZOOM, MEDIUM_ZOOM_THRESHOLD, MIN_ZOOM},
};
use crate::{
    helper::plugins::comms::ecs::event::GetTileUpdates,
    scene::explorer::ecs::{
        component::{AnimationIndicesComp, AnimationTimerComp},
        hard::{
            ANIMATED_SPRITE_Z, BUILDING_CHUNK_SIZE, BUILDING_DESPAWN_RANGE_MULTIPLIER,
            SLIM_BUILDING_CHUNK_SIZE, SLIM_CLOSE_ZOOM_THRESHOLD, SLIM_MAX_ZOOM,
            SLIM_MEDIUM_ZOOM_THRESHOLD, SLIM_TEXT_CHUNK_SIZE, SLIM_TILE_CHUNK_SIZE,
            TEXT_CHUNK_SIZE, TEXT_DESPAWN_RANGE_MULTIPLIER, TILE_CHUNK_SIZE,
            TILE_DESPAWN_RANGE_MULTIPLIER, TILE_SIZE,
        },
        resource::{
            ChunkTypeNumsRes, DespawnBuildingRangeRes, DespawnTextRangeRes, DespawnTileRangeRes,
            SpriteSheetBuildingRes, ZoomLevelNumsRes,
        },
        state::{ExplorerSubState, InitSpawnTileMapState},
    },
};

pub fn setup_animation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("we got to explorer init setup");
    let texture = asset_server.load("spritesheet/gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndicesComp { first: 1, last: 6 };
    commands.spawn((
        Sprite {
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            }),
            color: Color::Srgba(Color::BLACK.into()),
            image: texture,
            ..default()
        },
        Transform::from_translation(Vec3 {
            x: 0.0,
            y: 0.0,
            z: ANIMATED_SPRITE_Z,
        }),
        animation_indices,
        AnimationTimerComp(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

pub fn init_startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut init_map_state: ResMut<NextState<InitSpawnTileMapState>>,
    mut explorer_sub_state: ResMut<NextState<ExplorerSubState>>,
    full_map_state: Res<State<FullMapState>>,
    mut get_tiles: EventWriter<GetTileUpdates>,
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

    if *full_map_state == FullMapState::On {
        commands.insert_resource(ZoomLevelNumsRes {
            max_zoom: MAX_ZOOM,
            min_zoom: MIN_ZOOM,
            close_threshold: CLOSE_ZOOM_THRESHOLD,
            medium_threshold: MEDIUM_ZOOM_THRESHOLD,
        });
        commands.insert_resource(ChunkTypeNumsRes {
            building: BUILDING_CHUNK_SIZE.into(),
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

    init_map_state.set(InitSpawnTileMapState::Running);
    explorer_sub_state.set(ExplorerSubState::Running);
    // if data already is loaded from local then TS otherwise Height.
    get_tiles.send(GetTileUpdates(GetTileType::Height));
}
