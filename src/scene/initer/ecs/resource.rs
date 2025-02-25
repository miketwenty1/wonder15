use bevy::prelude::*;
use serde::Deserialize;

use crate::{
    helper::utils::funs::{format_sats, format_time},
    scene::explorer::{ecs::event::SwapTilesEvent, ui::ecs::state::ColorBlockchainKeySubState},
};

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
    pub light_blue: Color,
    pub blue: Color,
    pub dark_blue: Color,
    pub darker_blue: Color,
    pub cyan: Color,
    pub dark_cyan: Color,
    pub light_green: Color,
    pub green: Color,
    pub dark_green: Color,
    pub green_color: Color,
    pub red: Color,
    pub dark_red: Color,
    pub orange: Color,
    pub dark_orange: Color,
    pub pink: Color,
    pub dark_pink: Color,
    pub light_purple: Color,
    pub purple: Color,
    pub dark_purple: Color,
    pub hot_pink: Color,
    pub teal: Color,
    pub lavender: Color,
    pub navy: Color,
    pub light_brown: Color,
    pub brown: Color,
    pub llmagenta: Color,
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct BlockchainFilterKeys {
    pub fee: FilterLegend,
    pub block_time: FilterLegend,
    pub tx_count: FilterLegend,
    pub byte: FilterLegend,
    pub weight: FilterLegend,
    pub tgt_diff: FilterLegend,
    pub leading_zeros: FilterLegend,
    pub excess_work: FilterLegend,
    pub version: FilterLegend,
}

impl BlockchainFilterKeys {
    pub fn get_filter(&self, event: SwapTilesEvent) -> Result<&FilterLegend, &'static str> {
        match event {
            SwapTilesEvent::Fee => Ok(&self.fee),
            SwapTilesEvent::BlockTime => Ok(&self.block_time),
            SwapTilesEvent::TxCount => Ok(&self.tx_count),
            SwapTilesEvent::Byte => Ok(&self.byte),
            SwapTilesEvent::Weight => Ok(&self.weight),
            SwapTilesEvent::TargetDifficulty => Ok(&self.tgt_diff),
            SwapTilesEvent::LeadingZeros => Ok(&self.leading_zeros),
            SwapTilesEvent::ExcessWork => Ok(&self.excess_work),
            SwapTilesEvent::Version => Ok(&self.version),
            SwapTilesEvent::PlayerColor | SwapTilesEvent::Land | SwapTilesEvent::Iter => {
                Err("No equivalent FilterLegend for this event")
            }
        }
    }
    pub fn get_custom_string(&self, event: SwapTilesEvent) -> String {
        match event {
            SwapTilesEvent::PlayerColor => "Player Colors".to_owned(),
            SwapTilesEvent::Land => "Game Land".to_owned(),
            SwapTilesEvent::Fee => "Fees Per Block".to_owned(),
            SwapTilesEvent::BlockTime => "Blocktime Per Block".to_owned(),
            SwapTilesEvent::TxCount => "Transactions Per Block".to_owned(),
            SwapTilesEvent::Byte => "Size of Block in Bytes".to_owned(),
            SwapTilesEvent::Weight => "Size of Block in vBytes".to_owned(),
            SwapTilesEvent::TargetDifficulty => "TargetDifficulty of Block".to_owned(),
            SwapTilesEvent::LeadingZeros => "Leading Zeros of Block Hash".to_owned(),
            SwapTilesEvent::ExcessWork => "Excess Work of Block".to_owned(),
            SwapTilesEvent::Version => "Version Header of Block".to_owned(),
            SwapTilesEvent::Iter => "Iter".to_owned(),
        }
    }
    pub fn get_substate(
        &self,
        event: SwapTilesEvent,
    ) -> Result<ColorBlockchainKeySubState, &'static str> {
        match event {
            SwapTilesEvent::Fee => Ok(ColorBlockchainKeySubState::Fee),
            SwapTilesEvent::BlockTime => Ok(ColorBlockchainKeySubState::BlockTime),
            SwapTilesEvent::TxCount => Ok(ColorBlockchainKeySubState::TxCount),
            SwapTilesEvent::Byte => Ok(ColorBlockchainKeySubState::Byte),
            SwapTilesEvent::Weight => Ok(ColorBlockchainKeySubState::Weight),
            SwapTilesEvent::TargetDifficulty => Ok(ColorBlockchainKeySubState::TargetDifficulty),
            SwapTilesEvent::LeadingZeros => Ok(ColorBlockchainKeySubState::LeadingZeros),
            SwapTilesEvent::ExcessWork => Ok(ColorBlockchainKeySubState::ExcessWork),
            SwapTilesEvent::Version => Ok(ColorBlockchainKeySubState::Version),
            SwapTilesEvent::PlayerColor | SwapTilesEvent::Land | SwapTilesEvent::Iter => {
                Err("No equivalent ColorBlockchainKeySubState for this event")
            }
        }
    }
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct KeyColorRange {
    pub start: (i64, Color),
    pub end: (i64, Color),
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub enum FormatType {
    #[default]
    Sats,
    Time,
    //Count,
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct FilterLegend {
    pub vec: Vec<KeyColorRange>,
    pub format_type: FormatType,
}

impl FilterLegend {
    pub fn color_for_ranges(&self, n: i64) -> Option<Srgba> {
        for range in &self.vec {
            if let Some(color) = range.color_for(n) {
                return Some(color);
            }
        }
        None
    }
    pub fn format_value(&self, value: i64) -> (String, String) {
        match self.format_type {
            FormatType::Sats => format_sats(value),
            FormatType::Time => format_time(value),
        }
    }
}

impl KeyColorRange {
    pub fn new(start: i64, start_color: Color, end: i64, end_color: Color) -> Self {
        Self {
            start: (start, start_color),
            end: (end, end_color),
        }
    }

    fn color_for(&self, n: i64) -> Option<Srgba> {
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
}
