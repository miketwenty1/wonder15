use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    utils::HashMap,
};

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

pub fn bits_to_target_hash(bits: u32) -> usize {
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

pub fn make_gradient_image(
    images: &mut Assets<Image>,
    width: u32,
    height: u32,
    color_left: Color,
    color_right: Color,
) -> Handle<Image> {
    let mut data = vec![0; (width * height * 4) as usize];

    for y in 0..height {
        for x in 0..width {
            let t = x as f32 / (width - 1) as f32;
            let c = if color_left == color_right {
                color_left
            } else {
                lerp_color(color_left, color_right, t)
            };

            let i = (y * width + x) as usize * 4;
            data[i] = (c.to_srgba().red * 255.0) as u8;
            data[i + 1] = (c.to_srgba().green * 255.0) as u8;
            data[i + 2] = (c.to_srgba().blue * 255.0) as u8;
            data[i + 3] = (c.to_srgba().alpha * 255.0) as u8;
        }
    }

    let mut image = Image::new_fill(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.data = data;

    images.add(image)
}

pub fn make_gradient_image_corners(
    images: &mut Assets<Image>,
    width: u32,
    height: u32,
    tl: Color,
    tr: Color,
    bl: Color,
    br: Color,
) -> Handle<Image> {
    let mut data = vec![0; (width * height * 4) as usize];

    for y in 0..height {
        let v = y as f32 / (height - 1) as f32;
        for x in 0..width {
            let u = x as f32 / (width - 1) as f32;
            let top = lerp_color(tl, tr, u);
            let bottom = lerp_color(bl, br, u);
            let color = lerp_color(top, bottom, v);
            let i = (y * width + x) as usize * 4;
            let srgba = color.to_srgba();
            data[i] = (srgba.red * 255.0) as u8;
            data[i + 1] = (srgba.green * 255.0) as u8;
            data[i + 2] = (srgba.blue * 255.0) as u8;
            data[i + 3] = (srgba.alpha * 255.0) as u8;
        }
    }

    let mut image = Image::new_fill(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.data = data;

    images.add(image)
}

pub fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let a = a.to_linear();
    let b = b.to_linear();
    Color::srgba(
        a.red + t * (b.red - a.red),
        a.green + t * (b.green - a.green),
        a.blue + t * (b.blue - a.blue),
        a.alpha + t * (b.alpha - a.alpha),
    )
}

pub fn format_sats<T: Into<i64>>(value: T) -> (String, String) {
    let n = value.into();
    let sign = if n < 0 { "-" } else { "" };
    let n = n.abs() as u64;

    if n <= 9_999 {
        (format!("{}{}", sign, n), "sats".to_string())
    } else if n <= 999_999 {
        (format!("{}{}K", sign, n / 1_000), "sats".to_string())
    } else if n <= 99_999_999 {
        (format!("{}{}M", sign, n / 1_000_000), "sats".to_string())
    } else {
        (format!("{}{}", sign, n / 100_000_000), "BTC".to_string())
    }
}

pub fn format_time(seconds: i64) -> (String, String) {
    if seconds < 60 {
        (seconds.to_string(), "secs".to_string())
    } else if seconds < 3600 {
        (format!("{}", seconds / 60), "mins".to_string())
    } else {
        (format!("{}", seconds / 3600), "hours".to_string())
    }
}

pub fn format_count(count: i64) -> (String, String) {
    (count.to_string(), String::new())
}

pub fn format_bytes(bytes: i64) -> (String, String) {
    if bytes < 1000 {
        (bytes.to_string(), "Bytes".to_string())
    } else if bytes < 1_000_000 {
        (format!("{}", bytes / 1000), "KB".to_string())
    } else {
        (format!("{}", bytes / 1_000_000), "MB".to_string())
    }
}
pub fn format_vbytes(bytes: i64) -> (String, String) {
    if bytes < 1000 {
        (bytes.to_string(), "vBytes".to_string())
    } else if bytes < 1_000_000 {
        (format!("{}", bytes / 1000), "vKB".to_string())
    } else {
        (format!("{}", bytes / 1_000_000), "vMB".to_string())
    }
}

pub fn format_bytes_string(bytes: u32) -> (String, String) {
    let mut b = bytes.to_ne_bytes();
    b.reverse();
    let formatted = format!("0x{:02X}{:02X}{:02X}{:02X}", b[0], b[1], b[2], b[3]);
    (formatted, "v".to_string())
}

pub fn format_percent(diff: i64) -> (String, String) {
    (diff.to_string(), "%".to_string())
}

// fn bits_to_target(bits: u32) -> f64 {
//     let exponent = (bits >> 24) as i32;
//     let mantissa = (bits & 0x007fffff) as f64;
//     if exponent <= 3 {
//         mantissa / 2_f64.powi(8 * (3 - exponent))
//     } else {
//         mantissa * 2_f64.powi(8 * (exponent - 3))
//     }
// }

// pub fn percent_change_of_difficulties(previous_bits: u32, current_bits: u32) -> i32 {
//     let target_prev = bits_to_target(previous_bits);
//     let target_curr = bits_to_target(current_bits);
//     // Since difficulty is the inverse of the target,
//     // an increase in difficulty means target decreases.
//     // Thus, ratio = target_prev / target_curr will be > 1 when difficulty increases.
//     let ratio = target_prev / target_curr;
//     ((ratio - 1.0) * 100.0).round() as i32
// }

fn bits_to_target(bits: u32) -> f64 {
    let exponent = (bits >> 24) as i32;
    let mantissa = (bits & 0x007fffff) as f64;
    if exponent <= 3 {
        mantissa / 2_f64.powi(8 * (3 - exponent))
    } else {
        mantissa * 2_f64.powi(8 * (exponent - 3))
    }
}

fn difficulty_from_bits(bits: u32) -> f64 {
    let max_target = bits_to_target(486604799);
    max_target / bits_to_target(bits)
}

pub fn percent_change_of_difficulties(previous_bits: u32, current_bits: u32) -> i32 {
    let diff_prev = difficulty_from_bits(previous_bits);
    let diff_curr = difficulty_from_bits(current_bits);
    // Calculate percentage change as (current/previous - 1)*100.
    ((diff_curr / diff_prev - 1.0) * 100.0).round() as i32
}
