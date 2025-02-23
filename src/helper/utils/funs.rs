use bevy::{prelude::*, utils::HashMap};

use chrono::{DateTime, Duration, Utc};
use rand::Rng;

use crate::ecs::{
    hard::LIGHTEST_TEXT,
    resource::{TileData, TileResource},
};

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

pub fn get_resource_for_tile(block_hash: &[u8]) -> TileResource {
    if block_hash.len() != 32 {
        info!("hlen is {}", block_hash.len());
        return TileResource::Unknown;
    }
    let last_two_num = block_hash.last().unwrap();
    // Get the last two characters
    //let last_two_chars = &block_hash[block_hash.len() - 2..];

    // Convert the last two characters to a number
    //let last_two_num = u8::from_str_radix(last_two_chars, 16).unwrap_or(255);

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

// pub fn get_land_index(
//     height: u32,
//     resource: &TileResource,
//     tile_map: Option<&hashbrown::HashMap<u32, TileData>>,
// ) -> usize {
//     match tile_map {
//         Some(s) => {
//             info!("defaulting to 22 for get_land_index");
//             22
//         }
//         None => resource.spritesheet_index_value(),
//     }
// }

pub fn vec_tile_updates_to_hashmap(vec: Vec<TileData>) -> HashMap<u32, TileData> {
    vec.into_iter().map(|s| (s.height, s)).collect()
}

pub fn hex_str_to_32_bytes(s: &str) -> [u8; 32] {
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap();
    }
    out
}

pub fn leading_zeros_in_32(bytes: &[u8; 32]) -> usize {
    let mut count = 0;
    for &b in bytes {
        if b == 0 {
            count += 8;
        } else {
            count += b.leading_zeros() as usize;
            break;
        }
    }
    count
}

pub fn trailing_zeros_in_32(bytes: &[u8; 32]) -> usize {
    let mut count = 0;
    for &b in bytes.iter().rev() {
        if b == 0 {
            count += 8;
        } else {
            count += b.trailing_zeros() as usize;
            break;
        }
    }
    count
}

pub fn bits_to_target_hash(bits: i64) -> usize {
    let exponent = ((bits >> 24) & 0xff) as u8;
    let mantissa = (bits & 0x00ff_ffff) as u32;
    let mantissa_shifted = (mantissa as u128) << (8 * (exponent.saturating_sub(3)));
    mantissa_shifted.leading_zeros() as usize
}

pub fn get_text_color_per_tile_color(c: &Color) -> Color {
    if c.to_srgba().red > LIGHTEST_TEXT.red
        && c.to_srgba().green > LIGHTEST_TEXT.green
        && c.to_srgba().blue > LIGHTEST_TEXT.blue
    {
        Color::Srgba(Srgba::BLACK)
    } else {
        Color::Srgba(Srgba::WHITE)
    }
}
