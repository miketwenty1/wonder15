use bevy::{
    color::palettes::css::{DARK_GREEN, MAROON},
    prelude::*,
    utils::HashMap,
};

use crate::{
    ecs::{
        resource::{BlockchainHeight, FullMapLength, GameStaticInputs},
        state::{ExplorerCommsSubState, SceneState},
    },
    scene::{
        explorer::ecs::hard::TILE_MAP_LENGTH,
        initer::ecs::{
            component::{AnimationIndicesComp, AnimationTimerComp},
            resource::{
                BlockchainFilterKeys, BlockchainKeyColorPalette, BuildingValueLevelMapper,
                FilterLegend, FormatType, KeyColorRange, UiColorPalette,
            },
        },
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
    //current_blockheight: Res<BlockchainHeight>,
    // mut comm_map_state: ResMut<NextState<ExplorerCommsSubState>>,
    // static_inputs: Res<GameStaticInputs>,
) {
    // let map_side_length = 1000; //((current_blockheight.0 as f64).sqrt().ceil()) as u32 + 2;
    commands.insert_resource(FullMapLength(TILE_MAP_LENGTH));
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

    let mut numbers_map = HashMap::new();
    numbers_map.insert(0, 0);
    numbers_map.insert(32, 1);
    numbers_map.insert(64, 1);
    numbers_map.insert(128, 2);
    numbers_map.insert(256, 3);
    numbers_map.insert(512, 4);
    numbers_map.insert(1024, 5);
    numbers_map.insert(2048, 6);
    numbers_map.insert(4096, 7);
    numbers_map.insert(8192, 8);
    numbers_map.insert(16384, 9);
    numbers_map.insert(32768, 10);
    numbers_map.insert(65536, 11);
    numbers_map.insert(131072, 11);
    numbers_map.insert(262144, 11);
    numbers_map.insert(524288, 11);
    numbers_map.insert(1048576, 11);
    numbers_map.insert(2097152, 11);
    numbers_map.insert(4194304, 11);
    numbers_map.insert(8388608, 11);

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
        light_blue: Srgba::hex("ADD8E6").unwrap().into(),
        blue: Srgba::hex("0000FF").unwrap().into(),
        dark_blue: Srgba::hex("00008B").unwrap().into(),
        darker_blue: Srgba::hex("080848").unwrap().into(),
        cyan: Srgba::hex("00FFFF").unwrap().into(),
        dark_cyan: Srgba::hex("008B8B").unwrap().into(),
        light_green: Srgba::hex("90EE90").unwrap().into(),
        green: Srgba::hex("008000").unwrap().into(),
        dark_green: Srgba::hex("006400").unwrap().into(),
        green_color: Srgba::hex("00FF00").unwrap().into(),
        red: Srgba::hex("FF0000").unwrap().into(),
        dark_red: Srgba::hex("8B0000").unwrap().into(),
        orange: Srgba::hex("FFA500").unwrap().into(),
        dark_orange: Srgba::hex("FF8C00").unwrap().into(),
        pink: Srgba::hex("FFC0CB").unwrap().into(),
        dark_pink: Srgba::hex("FF1493").unwrap().into(),
        light_purple: Srgba::hex("CBC3E3").unwrap().into(),
        purple: Srgba::hex("A020F0").unwrap().into(),
        dark_purple: Srgba::hex("301934").unwrap().into(),

        hot_pink: Srgba::hex("FF69B4").unwrap().into(),
        teal: Srgba::hex("008080").unwrap().into(),
        lavender: Srgba::hex("E6E6FA").unwrap().into(),
        navy: Srgba::hex("000080").unwrap().into(),
        light_brown: Srgba::hex("A36F40").unwrap().into(),
        brown: Srgba::hex("A52A2A").unwrap().into(),
        llmagenta: Srgba::hex("FEC3EE").unwrap().into(),
    };

    let mut fee_hm = Vec::new();
    let mut block_time_hm = Vec::new();
    let mut tx_count_hm = Vec::new();
    let mut byte_hm = Vec::new();
    let mut weight_hm = Vec::new();
    //let mut tgt_diff_hm = Vec::new();
    let mut tgt_diff_diff_hm = Vec::new();
    let mut leading_zeros_hm = Vec::new();
    let mut excess_work_hm = Vec::new();
    let mut version_hm = Vec::new();

    // BLOCK FEE
    fee_hm.insert(0, KeyColorRange::new(0, bp.black, 0, bp.black));
    fee_hm.insert(1, KeyColorRange::new(0, bp.blue, 1_000_000, bp.light_green));
    fee_hm.insert(
        2,
        KeyColorRange::new(1_000_001, bp.light_green, 2_000_000, bp.yellow),
    );
    fee_hm.insert(
        3,
        KeyColorRange::new(2_000_000, bp.yellow, 5_000_000, bp.orange),
    );
    fee_hm.insert(
        4,
        KeyColorRange::new(5_000_000, bp.orange, 20_000_000, bp.red),
    );
    fee_hm.insert(
        5,
        KeyColorRange::new(20_000_000, bp.red, 100_000_000, bp.hot_pink),
    );
    fee_hm.insert(
        6,
        KeyColorRange::new(100_000_000, bp.hot_pink, 500_000_000, bp.magenta),
    );
    fee_hm.insert(
        7,
        KeyColorRange::new(500_000_000, bp.magenta, 1_000_000_000, bp.light_purple),
    );
    fee_hm.insert(
        8,
        KeyColorRange::new(1_000_000_000, bp.white, i64::MAX, bp.white),
    );

    // BLOCK TIME

    block_time_hm.insert(0, KeyColorRange::new(i64::MIN, bp.white, -5_000, bp.white));
    block_time_hm.insert(1, KeyColorRange::new(-5_000, bp.light_blue, 0, bp.blue));
    block_time_hm.insert(
        2,
        KeyColorRange::new(0, bp.light_green, 3_600, bp.dark_green),
    );
    block_time_hm.insert(
        3,
        KeyColorRange::new(3_600, bp.yellow, 7_200, bp.dark_yellow),
    );
    block_time_hm.insert(
        4,
        KeyColorRange::new(7_200, bp.dark_yellow, 10_800, bp.orange),
    );
    block_time_hm.insert(5, KeyColorRange::new(10_800, bp.orange, 21_600, bp.red));
    block_time_hm.insert(6, KeyColorRange::new(21_600, bp.red, 43_200, bp.pink));
    block_time_hm.insert(7, KeyColorRange::new(43_200, bp.pink, i64::MAX, bp.magenta));

    // TX COUNT

    tx_count_hm.insert(0, KeyColorRange::new(1, bp.black, 1, bp.black));
    tx_count_hm.insert(1, KeyColorRange::new(1, bp.green, 100, bp.light_green));
    tx_count_hm.insert(2, KeyColorRange::new(100, bp.blue, 1000, bp.light_blue));
    tx_count_hm.insert(3, KeyColorRange::new(1000, bp.yellow, 3000, bp.orange));
    tx_count_hm.insert(4, KeyColorRange::new(3000, bp.orange, 6000, bp.red));
    tx_count_hm.insert(5, KeyColorRange::new(6000, bp.red, 9000, bp.pink));
    tx_count_hm.insert(6, KeyColorRange::new(9000, bp.pink, i64::MAX, bp.white));

    // BYTES

    byte_hm.insert(0, KeyColorRange::new(0, bp.black, 200, bp.black));
    byte_hm.insert(1, KeyColorRange::new(200, bp.light_green, 10_000, bp.green));
    byte_hm.insert(
        2,
        KeyColorRange::new(10_000, bp.light_blue, 50_000, bp.blue),
    );
    byte_hm.insert(3, KeyColorRange::new(50_000, bp.yellow, 400_000, bp.orange));
    byte_hm.insert(4, KeyColorRange::new(400_000, bp.red, 600_000, bp.hot_pink));
    byte_hm.insert(
        5,
        KeyColorRange::new(600_000, bp.light_purple, 800_000, bp.magenta),
    );
    byte_hm.insert(
        6,
        KeyColorRange::new(800_000, bp.magenta, 999_999, bp.llmagenta),
    );
    byte_hm.insert(
        7,
        KeyColorRange::new(1_000_000, bp.white, i64::MAX, bp.white),
    );

    // WEIGHT

    weight_hm.insert(0, KeyColorRange::new(0, bp.black, 800, bp.black));
    weight_hm.insert(1, KeyColorRange::new(800, bp.light_green, 40_000, bp.green));
    weight_hm.insert(
        2,
        KeyColorRange::new(40_000, bp.light_blue, 200_000, bp.blue),
    );
    weight_hm.insert(
        3,
        KeyColorRange::new(200_000, bp.yellow, 1_600_000, bp.orange),
    );
    weight_hm.insert(
        4,
        KeyColorRange::new(1_600_000, bp.red, 2_400_000, bp.hot_pink),
    );
    weight_hm.insert(
        5,
        KeyColorRange::new(2_400_000, bp.light_purple, 3_200_000, bp.magenta),
    );
    weight_hm.insert(
        6,
        KeyColorRange::new(3_200_000, bp.magenta, 3_999_999, bp.llmagenta),
    );
    weight_hm.insert(
        7,
        KeyColorRange::new(4_000_000, bp.white, i64::MAX, bp.white),
    );

    // TGT DIFF DIFF

    tgt_diff_diff_hm.insert(
        0,
        KeyColorRange::new(i64::MIN, bp.dark_red, -75, bp.dark_red),
    );
    tgt_diff_diff_hm.insert(1, KeyColorRange::new(-75, bp.red, -20, bp.red));
    tgt_diff_diff_hm.insert(
        2,
        KeyColorRange::new(-20, bp.dark_orange, -5, bp.dark_orange),
    );
    tgt_diff_diff_hm.insert(3, KeyColorRange::new(-5, bp.orange, -1, bp.orange));
    tgt_diff_diff_hm.insert(4, KeyColorRange::new(0, bp.black, 0, bp.black));
    tgt_diff_diff_hm.insert(5, KeyColorRange::new(1, bp.dark_blue, 5, bp.dark_blue));
    tgt_diff_diff_hm.insert(6, KeyColorRange::new(5, bp.blue, 10, bp.blue));
    tgt_diff_diff_hm.insert(7, KeyColorRange::new(10, bp.cyan, 30, bp.cyan));
    tgt_diff_diff_hm.insert(8, KeyColorRange::new(30, bp.pink, 50, bp.pink));
    tgt_diff_diff_hm.insert(9, KeyColorRange::new(50, bp.hot_pink, 100, bp.hot_pink));
    tgt_diff_diff_hm.insert(
        10,
        KeyColorRange::new(100, bp.dark_magenta, 200, bp.dark_magenta),
    );
    tgt_diff_diff_hm.insert(11, KeyColorRange::new(200, bp.magenta, 300, bp.magenta));
    tgt_diff_diff_hm.insert(
        12,
        KeyColorRange::new(300, bp.llmagenta, i64::MAX, bp.white),
    );

    // LEADING ZEROS

    leading_zeros_hm.insert(0, KeyColorRange::new(32, bp.black, 32, bp.black));
    leading_zeros_hm.insert(1, KeyColorRange::new(32, bp.light_green, 40, bp.green));
    leading_zeros_hm.insert(2, KeyColorRange::new(40, bp.light_blue, 50, bp.blue));
    leading_zeros_hm.insert(3, KeyColorRange::new(50, bp.yellow, 60, bp.orange));
    leading_zeros_hm.insert(4, KeyColorRange::new(60, bp.cyan, 70, bp.light_purple));
    leading_zeros_hm.insert(
        5,
        KeyColorRange::new(70, bp.light_purple, 80, bp.dark_purple),
    );
    leading_zeros_hm.insert(6, KeyColorRange::new(80, bp.dark_purple, 90, bp.red));
    leading_zeros_hm.insert(7, KeyColorRange::new(90, bp.dark_orange, 95, bp.dark_red));
    leading_zeros_hm.insert(8, KeyColorRange::new(95, bp.hot_pink, 100, bp.magenta));
    leading_zeros_hm.insert(9, KeyColorRange::new(100, bp.magenta, 256, bp.white));

    // EXCESS WORK

    excess_work_hm.insert(0, KeyColorRange::new(0, bp.black, 6, bp.dark_blue));
    excess_work_hm.insert(1, KeyColorRange::new(6, bp.purple, 10, bp.purple));
    excess_work_hm.insert(2, KeyColorRange::new(10, bp.orange, 12, bp.orange));
    excess_work_hm.insert(3, KeyColorRange::new(12, bp.red, 14, bp.red));
    excess_work_hm.insert(4, KeyColorRange::new(14, bp.white, i64::MAX, bp.white));

    // VERSION

    version_hm.insert(0, KeyColorRange::new(1, bp.light_brown, 1, bp.light_brown));
    version_hm.insert(1, KeyColorRange::new(2, bp.lavender, 2, bp.lavender));
    version_hm.insert(
        2,
        KeyColorRange::new(3, bp.dark_magenta, 3, bp.dark_magenta),
    );
    version_hm.insert(
        3,
        KeyColorRange::new(4, bp.light_purple, 4, bp.light_purple),
    );
    version_hm.insert(
        4,
        KeyColorRange::new(536870912, MAROON.into(), 536870912, MAROON.into()),
    );
    version_hm.insert(
        5,
        KeyColorRange::new(541065216, bp.light_green, 541065216, bp.light_green),
    );
    version_hm.insert(
        6,
        KeyColorRange::new(536870914, bp.orange, 536870914, bp.orange),
    );
    version_hm.insert(7, KeyColorRange::new(549453824, bp.red, 549453824, bp.red));
    version_hm.insert(
        8,
        KeyColorRange::new(545259520, bp.hot_pink, 545259520, bp.hot_pink),
    );
    version_hm.insert(
        9,
        KeyColorRange::new(536870916, bp.pink, 536870916, bp.pink),
    );
    version_hm.insert(
        10,
        KeyColorRange::new(1073733632, bp.yellow, 1073733632, bp.yellow),
    );

    let blockchain_value_keys = BlockchainFilterKeys {
        fee: FilterLegend {
            vec: fee_hm,
            format_type: FormatType::Sats,
        },
        block_time: FilterLegend {
            vec: block_time_hm,
            format_type: FormatType::Time,
        },
        tx_count: FilterLegend {
            vec: tx_count_hm,
            format_type: FormatType::Count,
        },
        byte: FilterLegend {
            vec: byte_hm,
            format_type: FormatType::Bytes,
        },
        weight: FilterLegend {
            vec: weight_hm,
            format_type: FormatType::VBytes,
        },
        tgt_diff_diff: FilterLegend {
            vec: tgt_diff_diff_hm,
            format_type: FormatType::Percent,
        },
        leading_zeros: FilterLegend {
            vec: leading_zeros_hm,
            format_type: FormatType::Count,
        },
        excess_work: FilterLegend {
            vec: excess_work_hm,
            format_type: FormatType::Count,
        },
        version: FilterLegend {
            vec: version_hm,
            format_type: FormatType::ByteString,
        },
    };
    commands.insert_resource(bp);
    commands.insert_resource(ui_color_palette);
    commands.insert_resource(blockchain_value_keys);
    commands.insert_resource(BuildingValueLevelMapper(numbers_map));

    info!("we are about to set to Explorer!");
    scene_state.set(SceneState::Explorer);
    // need to set to Ts if cache was found
    // comm_map_state.set(ExplorerCommsSubState::Height);
}
