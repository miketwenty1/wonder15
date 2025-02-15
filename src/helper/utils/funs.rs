use bevy::{color::Srgba, log::info, utils::hashbrown};
use chrono::{DateTime, Duration, Utc};
use rand::Rng;

use crate::ecs::resource::{TileData, TileResource};

pub fn get_random_color() -> Srgba {
    let mut rng = rand::thread_rng();
    let r: f32 = rng.gen_range(0.0..1.0);
    let g: f32 = rng.gen_range(0.0..1.0);
    let b: f32 = rng.gen_range(0.0..1.0);

    //info!("getting a random color: {}-{}-{}", r, g, b);
    Srgba {
        red: r,
        green: g,
        blue: b,
        alpha: 1.0,
    }
}

pub fn to_millisecond_precision(dt: DateTime<Utc>) -> DateTime<Utc> {
    // Get the total number of milliseconds in the current second
    let milliseconds = dt.timestamp_subsec_millis();

    // Calculate the difference in microseconds to subtract
    let micros_to_subtract = dt.timestamp_subsec_micros() - (milliseconds * 1_000);

    // Subtract the extra microseconds to align to milliseconds

    dt - Duration::microseconds(micros_to_subtract as i64)
}

pub fn get_resource_for_tile(block_hash: &str) -> TileResource {
    // Ensure the block_hash is at least 2 characters long
    if block_hash.len() != 64 {
        return TileResource::Unknown;
    }

    // Get the last two characters
    let last_two_chars = &block_hash[block_hash.len() - 2..];

    // Convert the last two characters to a number
    let last_two_num = u8::from_str_radix(last_two_chars, 16).unwrap_or(255);
    // info!("last 2 nums of hash {:?}", last_two_num);
    // Match the number to the corresponding TileResource using ranges
    match last_two_num {
        0..=2 => TileResource::Mountain,
        3..=5 => TileResource::Water,
        6..=210 => TileResource::Grass,
        211..=252 => TileResource::Forest,
        253..=255 => TileResource::Desert,
        // _ => TileResource::Unknown, // Handle any unexpected characters
    }
}

pub fn get_land_index(
    height: u32,
    resource: &TileResource,
    tile_map: Option<&hashbrown::HashMap<u32, TileData>>,
) -> usize {
    match tile_map {
        Some(s) => {
            info!("defaulting to 22 for get_land_index");
            22
        }
        None => resource.spritesheet_index_value(),
    }
}
