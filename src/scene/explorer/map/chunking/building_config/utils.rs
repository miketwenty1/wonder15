use bevy::color::{Color, Srgba};

use crate::scene::explorer::ecs::hard::DARKEST_ALLOWED_BUILDING;
pub fn sanitize_building_color(c: Srgba) -> Srgba {
    if c.red < DARKEST_ALLOWED_BUILDING.red
        && c.green < DARKEST_ALLOWED_BUILDING.green
        && c.blue < DARKEST_ALLOWED_BUILDING.blue
    {
        return DARKEST_ALLOWED_BUILDING;
    }
    c
}

// pub fn get_text_color(c: &Color) -> Color {
//     if c.to_srgba().red > LIGHTEST_TEXT.red
//         && c.to_srgba().green > LIGHTEST_TEXT.green
//         && c.to_srgba().blue > LIGHTEST_TEXT.blue
//     {
//         Color::Srgba(Srgba::BLACK)
//     } else {
//         Color::Srgba(Srgba::WHITE)
//     }
// }
