use bevy::{prelude::*, text::FontSmoothing};

use crate::scene::{
    explorer::{ecs::component::GeneralUiBtn, ui::components::ToggleBlockchainBtn},
    initer::ecs::resource::UiColorPalette,
};

#[warn(clippy::too_many_arguments)]
pub fn spawn_game_toggle_button<T: Component>(
    parent: &mut ChildSpawnerCommands,
    toggle_btn_type: T,
    btn_text: &str,
    colors: &UiColorPalette,
    font: &Handle<Font>,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(120.),
                height: Val::Px(40.),
                border: UiRect::all(Val::Px(2.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                justify_items: JustifyItems::Center,
                align_content: AlignContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors.button_color),
            BorderColor(colors.node_color),
            BorderRadius::all(Val::Px(4.0)),
            toggle_btn_type,
            ToggleBlockchainBtn,
            GeneralUiBtn,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(btn_text),
                TextFont {
                    font: font.clone(),
                    font_size: 13.,
                    font_smoothing: FontSmoothing::AntiAliased,
                    ..default()
                },
                TextColor(colors.text_color),
            ));
        });
}
