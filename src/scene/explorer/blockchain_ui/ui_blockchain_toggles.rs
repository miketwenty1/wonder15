use bevy::prelude::*;

use crate::scene::{
    explorer::ecs::component::{
        BlockTimeToggleBtn, BlockchainFilterToggleParent, ByteToggleBtn, ExcessWorkToggleBtn,
        FeeToggleBtn, LeadZerosToggleBtn, TgtDiffDiffToggleBtn, TgtDiffToggleBtn, TxCountToggleBtn,
        VersionToggleBtn,
    },
    initer::ecs::resource::UiColorPalette,
};

use super::{components::ExplorerUiNodeLeft, toggle_button::spawn_game_toggle_button};

pub fn left_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    placement_query: Query<Entity, With<ExplorerUiNodeLeft>>,
    colors: Res<UiColorPalette>,
) {
    for ent in placement_query.iter() {
        info!("spawning right left ui");
        let mut side_parent = commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // align_items: AlignItems::Start,
                // justify_items: JustifyItems::Start,
                // justify_content: JustifyContent::Start,
                // align_content: AlignContent::Start,
                // align_self: AlignSelf::Start,
                // justify_self: JustifySelf::Start,
                flex_direction: FlexDirection::Column,
                margin: UiRect::bottom(Val::Auto),
                display: Display::None,
                ..default()
            },
            BackgroundColor(colors.node_color),
            BorderRadius::all(Val::Px(4.0)),
            BlockchainFilterToggleParent,
        ));
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");

        side_parent.with_children(|parent| {
            spawn_game_toggle_button(parent, FeeToggleBtn, "Fee", &colors, &font);
        });
        side_parent.with_children(|parent| {
            spawn_game_toggle_button(parent, BlockTimeToggleBtn, "BlockTime", &colors, &font);
        });
        side_parent.with_children(|parent| {
            spawn_game_toggle_button(parent, TxCountToggleBtn, "TxCount", &colors, &font);
        });
        side_parent.with_children(|parent| {
            spawn_game_toggle_button(parent, ByteToggleBtn, "Block Size", &colors, &font);
        });
        // side_parent.with_children(|parent| {
        //     spawn_game_toggle_button(parent, WeightToggleBtn, "Weights", &colors, &font);
        // });
        side_parent.with_children(|parent| {
            spawn_game_toggle_button(
                parent,
                TgtDiffToggleBtn,
                "Target Difficulty",
                &colors,
                &font,
            );
        });
        side_parent.with_children(|parent| {
            spawn_game_toggle_button(
                parent,
                TgtDiffDiffToggleBtn,
                "Difficulty Changes",
                &colors,
                &font,
            );
        });
        side_parent.with_children(|parent| {
            spawn_game_toggle_button(parent, LeadZerosToggleBtn, "Leading Zeros", &colors, &font);
        });
        side_parent.with_children(|parent| {
            spawn_game_toggle_button(parent, ExcessWorkToggleBtn, "Excess Work", &colors, &font);
        });
        side_parent.with_children(|parent| {
            spawn_game_toggle_button(parent, VersionToggleBtn, "Version Header", &colors, &font);
        });

        side_parent.set_parent(ent);
    }
}
