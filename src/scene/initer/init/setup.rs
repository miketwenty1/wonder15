use bevy::{color::palettes::css::DARK_GREEN, prelude::*, utils::HashMap};

use crate::{
    ecs::{
        resource::{BlockchainHeight, FullMapLength},
        state::{ExplorerCommsSubState, SceneState},
    },
    scene::initer::ecs::{
        component::{AnimationIndicesComp, AnimationTimerComp},
        resource::{BlockchainKeyColorPalette, BlockchainKeyValues, KeyColorRange, UiColorPalette},
    },
};

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndicesComp, &mut AnimationTimerComp, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

pub fn setup_things(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut scene_state: ResMut<NextState<SceneState>>,
    //scene_state: Res<State<SceneState>>,
    current_blockheight: Res<BlockchainHeight>,
    mut comm_map_state: ResMut<NextState<ExplorerCommsSubState>>,
) {
    let map_side_length = ((current_blockheight.0 as f64).sqrt().ceil()) as u32 + 2;
    commands.insert_resource(FullMapLength(map_side_length));
    let texture = asset_server.load("spritesheet/gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndicesComp { first: 1, last: 6 };
    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        Transform::from_translation(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 6.0,
        }),
        animation_indices,
        AnimationTimerComp(Timer::from_seconds(0.1, TimerMode::Repeating)),
        StateScoped(SceneState::Init),
    ));

    info!("we are about to set to Explorer!");
    scene_state.set(SceneState::Explorer);
    // need to set to Ts if cache was found
    comm_map_state.set(ExplorerCommsSubState::Height);

    let ui_color_palette = UiColorPalette {
        node_color: Srgba::hex("222831").unwrap().into(),
        node_color_lighter: Srgba::hex("353d48").unwrap().into(),
        button_color: Srgba::hex("393E46").unwrap().into(),
        lite_button_color: Srgba::hex("6A7382").unwrap().into(),
        accent_color: Srgba::hex("00ADB5").unwrap().into(),
        light_color: Srgba::hex("EEEEEE").unwrap().into(),
        text_color: Srgba::hex("FAFAFA").unwrap().into(),
        red_color: Srgba::hex("B50800").unwrap().into(),
        yellow_color: Srgba::hex("ADB500").unwrap().into(),
        green_color: DARK_GREEN.into(),
    };

    let bp = BlockchainKeyColorPalette {
        black: Srgba::hex("000000").unwrap().into(),
        white: Srgba::hex("FFFFFF").unwrap().into(),
        magenta: Srgba::hex("FF00FF").unwrap().into(),
        dark_magenta: Srgba::hex("8B008B").unwrap().into(),
        yellow: Srgba::hex("FFFF00").unwrap().into(),
        dark_yellow: Srgba::hex("999900").unwrap().into(),
        blue: Srgba::hex("0000FF").unwrap().into(),
        dark_blue: Srgba::hex("00008B").unwrap().into(),
        cyan: Srgba::hex("00FFFF").unwrap().into(),
        dark_cyan: Srgba::hex("008B8B").unwrap().into(),
        green: Srgba::hex("008000").unwrap().into(),
        dark_green: Srgba::hex("006400").unwrap().into(),
        green_color: Srgba::hex("00FF00").unwrap().into(),
        red: Srgba::hex("FF0000").unwrap().into(),
        dark_red: Srgba::hex("8B0000").unwrap().into(),
        orange: Srgba::hex("FFA500").unwrap().into(),
        dark_orange: Srgba::hex("FF8C00").unwrap().into(),
        pink: Srgba::hex("FFC0CB").unwrap().into(),
        dark_pink: Srgba::hex("FF1493").unwrap().into(),
        purple: Srgba::hex("A020F0").unwrap().into(),
        dark_purple: Srgba::hex("301934").unwrap().into(),

        hot_pink: Srgba::hex("FF69B4").unwrap().into(),
        teal: Srgba::hex("008080").unwrap().into(),
        lavender: Srgba::hex("E6E6FA").unwrap().into(),
        navy: Srgba::hex("000080").unwrap().into(),
        brown: Srgba::hex("A52A2A").unwrap().into(),
    };

    let mut fee_hm = Vec::new();
    let mut block_time_hm = Vec::new();
    let mut tx_count_hm = Vec::new();
    let mut byte_hm = Vec::new();
    let mut weight_hm = Vec::new();
    let mut tgt_diff_hm = Vec::new();
    let mut leading_zeros_hm = Vec::new();
    let mut excess_work_hm = Vec::new();
    let mut version_hm = Vec::new();

    fee_hm.insert(0, KeyColorRange::new(0, bp.black, 0, bp.black));
    fee_hm.insert(1, KeyColorRange::new(1, bp.orange, 5_000_000, bp.red));
    fee_hm.insert(
        2,
        KeyColorRange::new(5_000_001, bp.red, 20_000_000, bp.pink),
    );
    fee_hm.insert(
        3,
        KeyColorRange::new(20_000_001, bp.pink, 100_000_000, bp.purple),
    );
    fee_hm.insert(
        4,
        KeyColorRange::new(100_000_001, bp.purple, 500_000_000, bp.magenta),
    );
    fee_hm.insert(
        5,
        KeyColorRange::new(500_000_001, bp.magenta, 3_000_000_000, bp.hot_pink),
    );
    fee_hm.insert(
        6,
        KeyColorRange::new(3_000_000_001, bp.magenta, 4_200_000_000, bp.white),
    );

    let blockchain_value_keys = BlockchainKeyValues {
        fee: fee_hm,
        block_time: block_time_hm,
        tx_count: tx_count_hm,
        byte: byte_hm,
        weight: weight_hm,
        tgt_diff: tgt_diff_hm,
        leading_zeros: leading_zeros_hm,
        excess_work: excess_work_hm,
        version: version_hm,
    };
    commands.insert_resource(bp);
    commands.insert_resource(ui_color_palette);
    commands.insert_resource(blockchain_value_keys);
}
