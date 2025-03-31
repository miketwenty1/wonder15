use bevy::prelude::*;
use utils::sanitize_building_color;

pub mod level1;
pub mod level10;
pub mod level11;
pub mod level2;
pub mod level3;
pub mod level4;
pub mod level5;
pub mod level6;
pub mod level7;
pub mod level8;
pub mod level9;

// pub mod draw_select_tile;
// pub mod select_tile;
pub mod utils;

pub mod building_templates;

pub fn spawn_tile_level(
    building_sprite_index: &u32,
    layout: &Handle<TextureAtlasLayout>,
    texture: &Handle<Image>,
    builder: &mut ChildBuilder,
    color_for_sprites: Color,
    height: u32,
) {
    let color_sanitized = Color::Srgba(sanitize_building_color(color_for_sprites.into()));
    match building_sprite_index {
        1 => {
            // level1::spawn(texture, layout, builder, color_sanitized);
        }
        2 => {
            level2::spawn(texture, layout, builder, color_sanitized);
        }
        3 => {
            level3::spawn(texture, layout, builder, color_sanitized);
        }
        4 => {
            level4::spawn(texture, layout, builder, color_sanitized, height);
        }
        5 => {
            level5::spawn(texture, layout, builder, color_sanitized, height);
        }
        6 => {
            level6::spawn(texture, layout, builder, color_sanitized, height);
        }
        7 => {
            level7::spawn(texture, layout, builder, color_sanitized, height);
        }
        8 => {
            level8::spawn(texture, layout, builder, color_sanitized, height);
        }
        9 => {
            level9::spawn(texture, layout, builder, color_sanitized, height);
        }
        10 => {
            level10::spawn(texture, layout, builder, color_sanitized, height);
        }
        11 => {
            level11::spawn(texture, layout, builder, color_sanitized, height);
        }
        // 100 => {
        //     select_tile::spawn(texture, layout, builder, locationcoord);
        // }
        // 101 => {
        //     draw_select_tile::spawn(texture, layout, builder, locationcoord, color_for_sprites);
        // }
        _ => {
            // do no ting
        }
    }
}
