use bevy::{prelude::*, utils::HashMap};
use serde::Deserialize;

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct UiColorPalette {
    pub node_color: Color,
    pub node_color_lighter: Color,
    pub lite_button_color: Color,
    pub button_color: Color,
    pub accent_color: Color,
    pub light_color: Color,
    pub text_color: Color,
    pub red_color: Color,
    pub yellow_color: Color,
    pub green_color: Color,
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct BlockchainKeyColorPalette {
    pub black: Color,
    pub white: Color,
    pub magenta: Color,
    pub dark_magenta: Color,
    pub yellow: Color,
    pub dark_yellow: Color,
    pub blue: Color,
    pub dark_blue: Color,
    pub cyan: Color,
    pub dark_cyan: Color,
    pub green: Color,
    pub dark_green: Color,
    pub green_color: Color,
    pub red: Color,
    pub dark_red: Color,
    pub orange: Color,
    pub dark_orange: Color,
    pub pink: Color,
    pub dark_pink: Color,
    pub purple: Color,
    pub dark_purple: Color,
    pub hot_pink: Color,
    pub teal: Color,
    pub lavender: Color,
    pub navy: Color,
    pub brown: Color,
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct BlockchainKeyValues {
    pub fee: Vec<KeyColorRange>,
    pub block_time: Vec<KeyColorRange>,
    pub tx_count: Vec<KeyColorRange>,
    pub byte: Vec<KeyColorRange>,
    pub weight: Vec<KeyColorRange>,
    pub tgt_diff: Vec<KeyColorRange>,
    pub leading_zeros: Vec<KeyColorRange>,
    pub excess_work: Vec<KeyColorRange>,
    pub version: Vec<KeyColorRange>,
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct KeyColorRange {
    pub start: (u32, Color),
    pub end: (u32, Color),
}
impl KeyColorRange {
    pub fn new(start: u32, start_color: Color, end: u32, end_color: Color) -> Self {
        Self {
            start: (start, start_color),
            end: (end, end_color),
        }
    }

    fn color_for(&self, n: u32) -> Option<Srgba> {
        // Compare n to self.start.0 / self.end.0 (the u32s)
        if n < self.start.0 || n > self.end.0 {
            return None;
        }

        let range_len = (self.end.0 - self.start.0) as f32;
        let fraction = if range_len == 0.0 {
            0.0
        } else {
            (n - self.start.0) as f32 / range_len
        };

        // Use self.start.1 / self.end.1 (the Colors) for interpolation
        let start_color = self.start.1;
        let end_color = self.end.1;

        let r = start_color.to_srgba().red
            + fraction * (end_color.to_srgba().red - start_color.to_srgba().red);
        let g = start_color.to_srgba().green
            + fraction * (end_color.to_srgba().green - start_color.to_srgba().green);
        let b = start_color.to_srgba().blue
            + fraction * (end_color.to_srgba().blue - start_color.to_srgba().blue);

        Some(Srgba {
            red: r,
            green: g,
            blue: b,
            alpha: 1.0,
        })
    }
    pub fn color_for_ranges(ranges: &[KeyColorRange], n: u32) -> Option<Srgba> {
        for range in ranges {
            if let Some(color) = range.color_for(n) {
                return Some(color);
            }
        }
        None
    }
}
