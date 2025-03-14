use bevy::{
    color::palettes::{
        css::WHITE,
        tailwind::{BLUE_300, GREEN_300, RED_300},
    },
    prelude::*,
    text::FontSmoothing,
};

use crate::{
    helper::utils::funs::{make_gradient_image, make_gradient_image_corners},
    scene::{
        explorer::{
            ecs::event::SwapTilesEvent,
            ui::{components::ExplorerUiNodeRight, ecs::state::ColorBlockchainKeySubState},
        },
        initer::ecs::resource::{BlockchainFilterKeys, FilterLegend, UiColorPalette},
    },
};

#[allow(clippy::too_many_arguments)]
pub fn spawn_legend_driver(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    placement_query: Query<Entity, With<ExplorerUiNodeRight>>,
    ui_colors: Res<UiColorPalette>,
    key_color_ranges: Res<BlockchainFilterKeys>,
    mut images: ResMut<Assets<Image>>,
    mut event: EventReader<SwapTilesEvent>,
) {
    for e in event.read() {
        let scope = key_color_ranges.get_substate(e.clone()).unwrap();
        let header = key_color_ranges.get_custom_string(e.clone());
        match *e {
            SwapTilesEvent::TargetDifficulty => {
                spawn_tgt_diff_legend(
                    &mut commands,
                    &asset_server,
                    &placement_query,
                    &ui_colors,
                    header,
                    scope,
                );
            }
            SwapTilesEvent::TargetDifficultyDiff => {
                let filter = key_color_ranges.get_filter(e.clone()).unwrap();
                spawn_tgt_diff_diff_legend(
                    &mut commands,
                    &asset_server,
                    &placement_query,
                    &ui_colors,
                    filter.clone(),
                    &mut images,
                    header,
                    scope,
                );
            }
            SwapTilesEvent::Version => {
                let filter = key_color_ranges.get_filter(e.clone()).unwrap();
                spawn_version_legend(
                    &mut commands,
                    &asset_server,
                    &placement_query,
                    &ui_colors,
                    filter.clone(),
                    &mut images,
                    header,
                    scope,
                );
            }
            SwapTilesEvent::Iter => {
                info!("do no ting");
            }
            _ => {
                let filter = key_color_ranges.get_filter(e.clone()).unwrap();
                spawn_legend(
                    &mut commands,
                    &asset_server,
                    &placement_query,
                    &ui_colors,
                    filter.clone(),
                    &mut images,
                    header,
                    scope,
                );
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_legend(
    commands: &mut Commands,
    asset_server: &AssetServer,
    placement_query: &Query<Entity, With<ExplorerUiNodeRight>>,
    ui_colors: &UiColorPalette,
    filter_legend: FilterLegend,
    images: &mut Assets<Image>,
    header: String,
    scope: ColorBlockchainKeySubState,
) {
    for ent in placement_query.iter() {
        let font_header = asset_server.load("fonts/FiraSans-Bold.ttf");
        let font_light = asset_server.load("fonts/FiraSans-Light.ttf");
        info!("spawning right side ui with {:?}", scope);
        let mut side_parent = commands.spawn((
            Node {
                justify_items: JustifyItems::End,
                justify_self: JustifySelf::End,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(8.0)),
                margin: UiRect::bottom(Val::Auto),
                //width: Val::Px(300.),
                ..default()
            },
            BackgroundColor(ui_colors.node_color),
            StateScoped(scope),
            BorderRadius::all(Val::Px(10.0)),
        ));

        side_parent.with_children(|parent| {
            parent
                .spawn((Node {
                    display: Display::Flex,
                    margin: UiRect {
                        left: Val::Px(4.),
                        right: Val::Px(4.),
                        top: Val::Px(2.),
                        bottom: Val::Px(6.),
                    },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },))
                .with_children(|builder| {
                    builder.spawn((
                        Text::new(header.to_string()),
                        TextFont {
                            font: font_header.clone(),
                            font_size: 15.,
                            font_smoothing: FontSmoothing::AntiAliased,
                        },
                        TextColor(ui_colors.text_color),
                    ));
                });
        });

        side_parent.with_children(|parent| {
            let mut entries = parent.spawn((Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.),

                // margin: UiRect::all(Val::Px(6.0)),
                ..default()
            },));

            for range in &filter_legend.vec {
                entries.with_children(|entry| {
                    let mut row = entry.spawn((Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,

                        // margin: UiRect::horizontal(Val::Px(6.0)),
                        //margin: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },));
                    row.with_children(|builder| {
                        let left_color = range.start.1;
                        let right_color = range.end.1;

                        let first_val_range = filter_legend.format_value(range.start.0);
                        let end_val_range = filter_legend.format_value(range.end.0);

                        let sat_string = if first_val_range.0 == end_val_range.0 {
                            format!("{} {}", first_val_range.0, first_val_range.1)
                        } else if range.end.0 == i64::MAX {
                            format!("{} {} +", first_val_range.0, first_val_range.1)
                        } else if range.start.0 == i64::MIN {
                            format!("(-neg) {} {}", end_val_range.0, end_val_range.1)
                        } else if range.end.0 < 0 {
                            format!(
                                "(-neg) {}{} {}",
                                first_val_range.0, end_val_range.0, end_val_range.1
                            )
                        } else {
                            format!(
                                "{}-{} {}",
                                first_val_range.0, end_val_range.0, end_val_range.1
                            )
                        };

                        let image_handle =
                            make_gradient_image(images, 40, 20, left_color, right_color);
                        builder.spawn((
                            ImageNode {
                                image: image_handle,
                                ..default()
                            },
                            Node {
                                display: Display::Flex,
                                ..default()
                            },
                        ));

                        builder
                            .spawn(Node {
                                display: Display::Flex,
                                margin: UiRect::horizontal(Val::Px(10.0)),
                                // align_items: AlignItems::Center,
                                justify_content: JustifyContent::End,

                                ..default()
                            })
                            .with_children(|inner| {
                                inner.spawn((
                                    Text::new(sat_string),
                                    TextFont {
                                        font: font_light.clone(),
                                        font_size: 15.,
                                        font_smoothing: FontSmoothing::AntiAliased,
                                    },
                                    TextColor(ui_colors.text_color),
                                ));
                            });
                    });
                });
            }
        });

        side_parent.set_parent(ent);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_tgt_diff_diff_legend(
    commands: &mut Commands,
    asset_server: &AssetServer,
    placement_query: &Query<Entity, With<ExplorerUiNodeRight>>,
    ui_colors: &UiColorPalette,
    filter_legend: FilterLegend,
    images: &mut Assets<Image>,
    header: String,
    scope: ColorBlockchainKeySubState,
) {
    for ent in placement_query.iter() {
        let font_header = asset_server.load("fonts/FiraSans-Bold.ttf");
        let font_light = asset_server.load("fonts/FiraSans-Light.ttf");
        info!("spawning right side ui with {:?}", scope);
        let mut side_parent = commands.spawn((
            Node {
                justify_items: JustifyItems::End,
                justify_self: JustifySelf::End,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(8.0)),
                margin: UiRect::bottom(Val::Auto),
                //width: Val::Px(300.),
                ..default()
            },
            BackgroundColor(ui_colors.node_color),
            StateScoped(scope),
            BorderRadius::all(Val::Px(10.0)),
        ));

        side_parent.with_children(|parent| {
            parent
                .spawn((Node {
                    display: Display::Flex,
                    margin: UiRect {
                        left: Val::Px(4.),
                        right: Val::Px(4.),
                        top: Val::Px(2.),
                        bottom: Val::Px(6.),
                    },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },))
                .with_children(|builder| {
                    builder.spawn((
                        Text::new(header.to_string()),
                        TextFont {
                            font: font_header.clone(),
                            font_size: 15.,
                            font_smoothing: FontSmoothing::AntiAliased,
                        },
                        TextColor(ui_colors.text_color),
                    ));
                });
        });

        side_parent.with_children(|parent| {
            let mut entries = parent.spawn((Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.),

                // margin: UiRect::all(Val::Px(6.0)),
                ..default()
            },));

            for range in &filter_legend.vec {
                entries.with_children(|entry| {
                    let mut row = entry.spawn((Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,

                        // margin: UiRect::horizontal(Val::Px(6.0)),
                        //margin: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },));
                    row.with_children(|builder| {
                        let left_color = range.start.1;
                        let right_color = range.end.1;

                        let first_val_range = filter_legend.format_value(range.start.0);
                        let end_val_range = filter_legend.format_value(range.end.0);

                        let sat_string = if first_val_range.0 == end_val_range.0 {
                            format!("{} {}", first_val_range.0, first_val_range.1)
                        } else if range.end.0 == i64::MAX {
                            format!("{} {} +", first_val_range.0, first_val_range.1)
                        } else if range.start.0 == i64::MIN {
                            format!("(-neg) {} {}", end_val_range.0, end_val_range.1)
                        } else if range.end.0 < 0 {
                            format!(
                                "(-neg) {}{} {}",
                                first_val_range.0, end_val_range.0, end_val_range.1
                            )
                        } else {
                            format!(
                                "{}-{} {}",
                                first_val_range.0, end_val_range.0, end_val_range.1
                            )
                        };

                        let image_handle =
                            make_gradient_image(images, 40, 20, left_color, right_color);
                        builder.spawn((
                            ImageNode {
                                image: image_handle,
                                ..default()
                            },
                            Node {
                                display: Display::Flex,
                                ..default()
                            },
                        ));

                        builder
                            .spawn(Node {
                                display: Display::Flex,
                                margin: UiRect::horizontal(Val::Px(10.0)),
                                // align_items: AlignItems::Center,
                                justify_content: JustifyContent::End,

                                ..default()
                            })
                            .with_children(|inner| {
                                inner.spawn((
                                    Text::new(sat_string),
                                    TextFont {
                                        font: font_light.clone(),
                                        font_size: 15.,
                                        font_smoothing: FontSmoothing::AntiAliased,
                                    },
                                    TextColor(ui_colors.text_color),
                                ));
                            });
                    });
                });
            }
        });

        side_parent.set_parent(ent);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_tgt_diff_legend(
    commands: &mut Commands,
    asset_server: &AssetServer,
    placement_query: &Query<Entity, With<ExplorerUiNodeRight>>,
    ui_colors: &UiColorPalette,
    header: String,
    scope: ColorBlockchainKeySubState,
) {
    for ent in placement_query.iter() {
        let font_header = asset_server.load("fonts/FiraSans-Bold.ttf");
        let font_light = asset_server.load("fonts/FiraSans-Light.ttf");
        info!("spawning right side ui with {:?}", scope);
        let mut side_parent = commands.spawn((
            Node {
                justify_items: JustifyItems::End,
                justify_self: JustifySelf::End,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(8.0)),
                margin: UiRect::bottom(Val::Auto),
                //width: Val::Px(300.),
                ..default()
            },
            BackgroundColor(ui_colors.node_color),
            StateScoped(scope),
            BorderRadius::all(Val::Px(10.0)),
        ));

        side_parent.with_children(|parent| {
            parent
                .spawn((Node {
                    display: Display::Flex,
                    margin: UiRect {
                        left: Val::Px(4.),
                        right: Val::Px(4.),
                        top: Val::Px(2.),
                        bottom: Val::Px(6.),
                    },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },))
                .with_children(|builder| {
                    builder.spawn((
                        Text::new(header.to_string()),
                        TextFont {
                            font: font_header.clone(),
                            font_size: 15.,
                            font_smoothing: FontSmoothing::AntiAliased,
                        },
                        TextColor(ui_colors.text_color),
                    ));
                });
        });

        side_parent.with_children(|parent| {
            parent
                .spawn((Node {
                    display: Display::Flex,
                    margin: UiRect {
                        left: Val::Px(4.),
                        right: Val::Px(4.),
                        top: Val::Px(2.),
                        bottom: Val::Px(6.),
                    },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },))
                .with_children(|builder| {
                    builder.spawn((
                        Text::new("Bitcoin's mining\ndifficulty recalculates\nevery 2016 blocks, or\nroughly every 2 weeks."),
                        TextFont {
                            font: font_light.clone(),
                            font_size: 15.,
                            font_smoothing: FontSmoothing::AntiAliased,
                        },
                        TextColor(ui_colors.text_color),
                    ));
                });
        });

        side_parent.set_parent(ent);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_version_legend(
    commands: &mut Commands,
    asset_server: &AssetServer,
    placement_query: &Query<Entity, With<ExplorerUiNodeRight>>,
    ui_colors: &UiColorPalette,
    filter_legend: FilterLegend,
    images: &mut Assets<Image>,
    header: String,
    scope: ColorBlockchainKeySubState,
) {
    for ent in placement_query.iter() {
        let font_header = asset_server.load("fonts/FiraSans-Bold.ttf");
        let font_light = asset_server.load("fonts/FiraSans-Light.ttf");
        info!("spawning right side ui with {:?}", scope);
        let mut side_parent = commands.spawn((
            Node {
                justify_items: JustifyItems::End,
                justify_self: JustifySelf::End,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(8.0)),
                margin: UiRect::bottom(Val::Auto),
                //width: Val::Px(300.),
                ..default()
            },
            BackgroundColor(ui_colors.node_color),
            StateScoped(scope),
            BorderRadius::all(Val::Px(10.0)),
        ));

        side_parent.with_children(|parent| {
            parent
                .spawn((Node {
                    display: Display::Flex,
                    margin: UiRect {
                        left: Val::Px(4.),
                        right: Val::Px(4.),
                        top: Val::Px(2.),
                        bottom: Val::Px(6.),
                    },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },))
                .with_children(|builder| {
                    builder.spawn((
                        Text::new(header.to_string()),
                        TextFont {
                            font: font_header.clone(),
                            font_size: 15.,
                            font_smoothing: FontSmoothing::AntiAliased,
                        },
                        TextColor(ui_colors.text_color),
                    ));
                });
        });

        side_parent.with_children(|parent| {
            let mut entries = parent.spawn((Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.),

                // margin: UiRect::all(Val::Px(6.0)),
                ..default()
            },));

            for range in &filter_legend.vec {
                entries.with_children(|entry| {
                    let mut row = entry.spawn((Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,

                        // margin: UiRect::horizontal(Val::Px(6.0)),
                        //margin: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },));
                    row.with_children(|builder| {
                        let left_color = range.start.1;
                        let right_color = range.end.1;

                        let first_val_range = filter_legend.format_value(range.start.0);
                        let end_val_range = filter_legend.format_value(range.end.0);

                        let sat_string = if first_val_range.0 == end_val_range.0 {
                            format!("{} {}", first_val_range.0, first_val_range.1)
                        } else if range.end.0 == i64::MAX {
                            format!("{} {} +", first_val_range.0, first_val_range.1)
                        } else if range.start.0 == i64::MIN {
                            format!("(-neg) {} {}", end_val_range.0, end_val_range.1)
                        } else if range.end.0 < 0 {
                            format!(
                                "(-neg) {}{} {}",
                                first_val_range.0, end_val_range.0, end_val_range.1
                            )
                        } else {
                            format!(
                                "{}-{} {}",
                                first_val_range.0, end_val_range.0, end_val_range.1
                            )
                        };

                        let image_handle =
                            make_gradient_image(images, 40, 20, left_color, right_color);
                        builder.spawn((
                            ImageNode {
                                image: image_handle,
                                ..default()
                            },
                            Node {
                                display: Display::Flex,
                                ..default()
                            },
                        ));

                        builder
                            .spawn(Node {
                                display: Display::Flex,
                                margin: UiRect::horizontal(Val::Px(10.0)),
                                // align_items: AlignItems::Center,
                                justify_content: JustifyContent::End,

                                ..default()
                            })
                            .with_children(|inner| {
                                inner.spawn((
                                    Text::new(sat_string),
                                    TextFont {
                                        font: font_light.clone(),
                                        font_size: 15.,
                                        font_smoothing: FontSmoothing::AntiAliased,
                                    },
                                    TextColor(ui_colors.text_color),
                                ));
                            });
                    });
                });
            }

            // CUSTOM COLOR to describe MISC color

            entries.with_children(|entry| {
                let mut row = entry.spawn((Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,

                    // margin: UiRect::horizontal(Val::Px(6.0)),
                    //margin: UiRect::all(Val::Px(2.0)),
                    ..default()
                },));
                row.with_children(|builder| {
                    let image_handle = make_gradient_image_corners(
                        images,
                        40,
                        20,
                        WHITE.into(),
                        GREEN_300.into(),
                        RED_300.into(),
                        BLUE_300.into(),
                    );
                    builder.spawn((
                        ImageNode {
                            image: image_handle,
                            ..default()
                        },
                        Node {
                            display: Display::Flex,
                            ..default()
                        },
                    ));

                    builder
                        .spawn(Node {
                            display: Display::Flex,
                            margin: UiRect::horizontal(Val::Px(10.0)),
                            // align_items: AlignItems::Center,
                            justify_content: JustifyContent::End,

                            ..default()
                        })
                        .with_children(|inner| {
                            inner.spawn((
                                Text::new("MISC VERSION"),
                                TextFont {
                                    font: font_light.clone(),
                                    font_size: 15.,
                                    font_smoothing: FontSmoothing::AntiAliased,
                                },
                                TextColor(ui_colors.text_color),
                            ));
                        });
                });
            });
        });

        side_parent.set_parent(ent);
    }
}
