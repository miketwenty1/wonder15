use bevy::prelude::*;

use crate::scene::{
    explorer::ecs::component::{
        BlockTimeToggleBtn, ByteToggleBtn, ExcessWorkToggleBtn, FeeToggleBtn, LeadZerosToggleBtn,
        TgtDiffToggleBtn, TxCountToggleBtn, VersionToggleBtn, WeightToggleBtn,
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
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BorderRadius::all(Val::Px(4.0)),
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
            spawn_game_toggle_button(parent, ByteToggleBtn, "Bytes", &colors, &font);
        });
        side_parent.with_children(|parent| {
            spawn_game_toggle_button(parent, WeightToggleBtn, "Weights", &colors, &font);
        });
        side_parent.with_children(|parent| {
            spawn_game_toggle_button(parent, TgtDiffToggleBtn, "TargetDifficulty", &colors, &font);
        });
        side_parent.with_children(|parent| {
            spawn_game_toggle_button(parent, LeadZerosToggleBtn, "LeadingZeros", &colors, &font);
        });
        side_parent.with_children(|parent| {
            spawn_game_toggle_button(parent, ExcessWorkToggleBtn, "ExcessWork", &colors, &font);
        });
        side_parent.with_children(|parent| {
            spawn_game_toggle_button(parent, VersionToggleBtn, "Version", &colors, &font);
        });

        side_parent.set_parent(ent);
    }
}
