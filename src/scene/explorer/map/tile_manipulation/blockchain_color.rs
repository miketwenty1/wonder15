use bevy::prelude::*;
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn get_bits_color(value: u32) -> Color {
    // Create a hasher and hash the value
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let hash = hasher.finish();

    // Extract RGB values from the hash
    let r = (hash & 0xFF) as f32 / 255.0;
    let g = ((hash >> 8) & 0xFF) as f32 / 255.0;
    let b = ((hash >> 16) & 0xFF) as f32 / 255.0;

    // Return the color
    Color::Srgba(Srgba {
        red: r,
        green: g,
        blue: b,
        alpha: 1.0,
    })
}
pub fn get_version_color(value: u32) -> Color {
    // Create a hasher and hash the value
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let hash = hasher.finish();

    // Extract RGB values from the hash
    let r = (hash & 0xFF) as f32 / 255.0;
    let g = ((hash >> 8) & 0xFF) as f32 / 255.0;
    let b = ((hash >> 16) & 0xFF) as f32 / 255.0;

    // Return the color
    Color::Srgba(Srgba {
        red: r,
        green: g,
        blue: b,
        alpha: 1.0,
    })
}
