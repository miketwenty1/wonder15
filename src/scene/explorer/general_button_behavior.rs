use bevy::prelude::*;

use crate::scene::initer::ecs::resource::UiColorPalette;

use super::ecs::component::GeneralUiBtn;

#[allow(clippy::type_complexity)]
pub fn general_btn(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<GeneralUiBtn>),
    >,
    colors: Res<UiColorPalette>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = colors.accent_color.into();
                border_color.0 = colors.green_color;
            }
            Interaction::Hovered => {
                *color = colors.lite_button_color.into();
                border_color.0 = colors.node_color_lighter;
            }
            Interaction::None => {
                *color = colors.button_color.into();
                border_color.0 = colors.node_color;
            }
        }
    }
}
