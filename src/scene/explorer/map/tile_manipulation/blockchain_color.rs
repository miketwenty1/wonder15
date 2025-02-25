use bevy::prelude::*;
use std::hash::{DefaultHasher, Hash, Hasher};

// pub fn get_fee_color(value: i64) -> Color {
//     match value {
//         // 0: Black
//         0 => Color::Srgba(Srgba {
//             red: 0.0,
//             green: 0.0,
//             blue: 0.0,
//             alpha: 1.0,
//         }),

//         // 1..=5_000,000: Orange to Red
//         1..=5_000_000 => {
//             let intensity = value as f32 / 5_000_000.0;
//             Color::Srgba(Srgba {
//                 red: 1.0,
//                 green: 0.5 - intensity * 0.5,
//                 blue: 0.0,
//                 alpha: 1.0,
//             })
//         }

//         // 5_000_001..=20_000_000: Red to Pink
//         5_000_001..=20_000_000 => {
//             let intensity = (value - 5_000_001) as f32 / 15_000_000.0;
//             Color::Srgba(Srgba {
//                 red: 1.0,
//                 green: 0.0,
//                 blue: intensity * 0.5,
//                 alpha: 1.0,
//             })
//         }

//         // 20_000_001..=100_000_000: Pink to Purpleish Blue
//         20_000_001..=100_000_000 => {
//             let intensity = (value - 20_000_001) as f32 / 80_000_000.0;
//             Color::Srgba(Srgba {
//                 red: 1.0 - intensity * 0.5,
//                 green: 0.0,
//                 blue: 0.5 + intensity * 0.5,
//                 alpha: 1.0,
//             })
//         }

//         // 100_000_001..=500_000_000: Purple to Magenta
//         100_000_001..=500_000_000 => {
//             let intensity = (value - 100_000_001) as f32 / 400_000_000.0;
//             Color::Srgba(Srgba {
//                 red: 0.5 + intensity * 0.5,
//                 green: 0.0,
//                 blue: 1.0 - intensity * 0.5,
//                 alpha: 1.0,
//             })
//         }

//         // 500_000_001..=3_000_000_000: Magenta to Hot Pink
//         500_000_001..=3_000_000_000 => {
//             let intensity = (value - 500_000_001) as f32 / 2_500_000_000.0;
//             Color::Srgba(Srgba {
//                 red: 1.0,
//                 green: 0.0 + intensity * 0.5,
//                 blue: 1.0 - intensity * 0.5,
//                 alpha: 1.0,
//             })
//         }

//         // 3_000_000_001+: White
//         _ => Color::Srgba(Srgba {
//             red: 1.0,
//             green: 1.0,
//             blue: 1.0,
//             alpha: 1.0,
//         }),
//     }
// }

// pub fn get_blocktime_color(value: i64) -> Color {
//     match value {
//         // -infinity to 0: White
//         i64::MIN..=0 => Color::Srgba(Srgba {
//             red: 1.0,
//             green: 1.0,
//             blue: 1.0,
//             alpha: 1.0,
//         }),

//         // 1 to 600: Light Green to Green
//         1..=600 => {
//             let intensity = value as f32 / 600.0;
//             Color::Srgba(Srgba {
//                 red: 0.0,
//                 green: 0.8 - intensity * 0.5,
//                 blue: 0.0,
//                 alpha: 1.0,
//             })
//         }

//         // 601 to 1800: Light Blue to Blue
//         601..=1800 => {
//             let intensity = (value - 601) as f32 / 1200.0;
//             Color::Srgba(Srgba {
//                 red: 0.0,
//                 green: 0.5 - intensity * 0.5,
//                 blue: 1.0,
//                 alpha: 1.0,
//             })
//         }

//         // 1801 to 3600: Yellow to Dark Yellow
//         1801..=3600 => {
//             let intensity = (value - 1801) as f32 / 1800.0;
//             Color::Srgba(Srgba {
//                 red: 1.0,
//                 green: 1.0 - intensity * 0.3,
//                 blue: 0.0,
//                 alpha: 1.0,
//             })
//         }

//         // 3601 to 7200: Light Orange to Orange
//         3601..=7200 => {
//             let intensity = (value - 3601) as f32 / 3600.0;
//             Color::Srgba(Srgba {
//                 red: 1.0,
//                 green: 0.5 - intensity * 0.5,
//                 blue: 0.0,
//                 alpha: 1.0,
//             })
//         }

//         // 7201 to 10800: Light Red to Dark Red
//         7201..=10800 => {
//             let intensity = (value - 7201) as f32 / 3600.0;
//             Color::Srgba(Srgba {
//                 red: 1.0,
//                 green: 0.0,
//                 blue: intensity * 0.2,
//                 alpha: 1.0,
//             })
//         }

//         // 10801+: Black
//         _ => Color::Srgba(Srgba {
//             red: 0.0,
//             green: 0.0,
//             blue: 0.0,
//             alpha: 1.0,
//         }),
//     }
// }

// pub fn get_tx_count_color(value: u32) -> Color {
//     match value {
//         // 1: Black
//         1 => Color::Srgba(Srgba {
//             red: 0.0,
//             green: 0.0,
//             blue: 0.0,
//             alpha: 1.0,
//         }),

//         // 2 to 100: Light Green to Green
//         2..=100 => {
//             let intensity = (value - 2) as f32 / 98.0;
//             Color::Srgba(Srgba {
//                 red: 0.0,
//                 green: 0.8 - intensity * 0.5,
//                 blue: 0.0,
//                 alpha: 1.0,
//             })
//         }

//         // 101 to 999: Light Blue to Blue
//         101..=999 => {
//             let intensity = (value - 101) as f32 / 898.0;
//             Color::Srgba(Srgba {
//                 red: 0.0,
//                 green: 0.5 - intensity * 0.5,
//                 blue: 1.0,
//                 alpha: 1.0,
//             })
//         }

//         // 1000 to 3000: Yellow to Orange
//         1000..=3000 => {
//             let intensity = (value - 1000) as f32 / 2000.0;
//             Color::Srgba(Srgba {
//                 red: 1.0,
//                 green: 1.0 - intensity * 0.5,
//                 blue: 0.0,
//                 alpha: 1.0,
//             })
//         }

//         // 3001 to 6000: Orange to Red
//         3001..=6000 => {
//             let intensity = (value - 3001) as f32 / 2999.0;
//             Color::Srgba(Srgba {
//                 red: 1.0,
//                 green: 0.5 - intensity * 0.5,
//                 blue: 0.0,
//                 alpha: 1.0,
//             })
//         }

//         // 6001 to 9000: Red to Pink
//         6001..=9000 => {
//             let intensity = (value - 6001) as f32 / 2999.0;
//             Color::Srgba(Srgba {
//                 red: 1.0,
//                 green: 0.0,
//                 blue: intensity * 0.5,
//                 alpha: 1.0,
//             })
//         }

//         // 9001+: White
//         _ => Color::Srgba(Srgba {
//             red: 1.0,
//             green: 1.0,
//             blue: 1.0,
//             alpha: 1.0,
//         }),
//     }
// }

// pub fn get_byte_color(value: u32) -> Color {
//     match value {
//         // 0 to 200: Black
//         0..=200 => Color::Srgba(Srgba {
//             red: 0.0,
//             green: 0.0,
//             blue: 0.0,
//             alpha: 1.0,
//         }),

//         // 201 to 10,000: Light Green to Green
//         201..=10_000 => {
//             let intensity = (value - 201) as f32 / 9_800.0;
//             Color::Srgba(Srgba {
//                 red: 0.0,
//                 green: 0.8 - intensity * 0.5,
//                 blue: 0.0,
//                 alpha: 1.0,
//             })
//         }

//         // 10,001 to 50,000: Light Blue to Blue
//         10_001..=50_000 => {
//             let intensity = (value - 10_001) as f32 / 39_999.0;
//             Color::Srgba(Srgba {
//                 red: 0.0,
//                 green: 0.5 - intensity * 0.5,
//                 blue: 1.0,
//                 alpha: 1.0,
//             })
//         }

//         // 50,001 to 200,000: Yellow to Light Orange
//         50_001..=200_000 => {
//             let intensity = (value - 50_001) as f32 / 149_999.0;
//             Color::Srgba(Srgba {
//                 red: 1.0,
//                 green: 1.0 - intensity * 0.2,
//                 blue: intensity * 0.2,
//                 alpha: 1.0,
//             })
//         }

//         // 200,001 to 400,000: Light Orange to Orange
//         200_001..=400_000 => {
//             let intensity = (value - 200_001) as f32 / 199_999.0;
//             Color::Srgba(Srgba {
//                 red: 1.0,
//                 green: 0.8 - intensity * 0.3,
//                 blue: 0.0,
//                 alpha: 1.0,
//             })
//         }

//         // 400,001 to 600,000: Light Red to Red
//         400_001..=600_000 => {
//             let intensity = (value - 400_001) as f32 / 199_999.0;
//             Color::Srgba(Srgba {
//                 red: 1.0,
//                 green: intensity * 0.5,
//                 blue: intensity * 0.5,
//                 alpha: 1.0,
//             })
//         }

//         // 600,001 to 800,000: Light Purple to Magenta
//         600_001..=800_000 => {
//             let intensity = (value - 600_001) as f32 / 199_999.0;
//             Color::Srgba(Srgba {
//                 red: 1.0,
//                 green: intensity * 0.0,
//                 blue: 1.0 - intensity,
//                 alpha: 1.0,
//             })
//         }

//         // 800,001 to 999,900: Light Magenta to Magenta
//         800_001..=999_900 => {
//             let intensity = (value - 800_001) as f32 / 199_899.0;
//             Color::Srgba(Srgba {
//                 red: 1.0,
//                 green: 0.0,
//                 blue: 1.0 - intensity * 0.5,
//                 alpha: 1.0,
//             })
//         }

//         // 999,901+: White
//         _ => Color::Srgba(Srgba {
//             red: 1.0,
//             green: 1.0,
//             blue: 1.0,
//             alpha: 1.0,
//         }),
//     }
// }

// pub fn get_weight_color(value: u32) -> Color {
//     match value {
//         // 0 to 800: Black
//         0..=800 => Color::Srgba(Srgba {
//             red: 0.0,
//             green: 0.0,
//             blue: 0.0,
//             alpha: 1.0,
//         }),

//         // 801 to 40,000: Light Green to Green
//         801..=40_000 => {
//             let intensity = (value - 801) as f32 / 39_199.0;
//             Color::Srgba(Srgba {
//                 red: 0.0,
//                 green: 0.8 - intensity * 0.5,
//                 blue: 0.0,
//                 alpha: 1.0,
//             })
//         }

//         // 40,001 to 200,000: Light Blue to Blue
//         40_001..=200_000 => {
//             let intensity = (value - 40_001) as f32 / 159_999.0;
//             Color::Srgba(Srgba {
//                 red: 0.0,
//                 green: 0.5 - intensity * 0.5,
//                 blue: 1.0,
//                 alpha: 1.0,
//             })
//         }

//         // 200,001 to 800,000: Yellow to Light Orange
//         200_001..=800_000 => {
//             let intensity = (value - 200_001) as f32 / 599_999.0;
//             Color::Srgba(Srgba {
//                 red: 1.0,
//                 green: 1.0 - intensity * 0.2,
//                 blue: intensity * 0.2,
//                 alpha: 1.0,
//             })
//         }

//         // 800,001 to 1,600,000: Light Orange to Orange
//         800_001..=1_600_000 => {
//             let intensity = (value - 800_001) as f32 / 799_999.0;
//             Color::Srgba(Srgba {
//                 red: 1.0,
//                 green: 0.8 - intensity * 0.3,
//                 blue: 0.0,
//                 alpha: 1.0,
//             })
//         }

//         // 1,600,001 to 2,400,000: Light Red to Red
//         1_600_001..=2_400_000 => {
//             let intensity = (value - 1_600_001) as f32 / 799_999.0;
//             Color::Srgba(Srgba {
//                 red: 1.0,
//                 green: intensity * 0.5,
//                 blue: intensity * 0.5,
//                 alpha: 1.0,
//             })
//         }

//         // 2,400,001 to 3,200,000: Light Purple to Magenta
//         2_400_001..=3_200_000 => {
//             let intensity = (value - 2_400_001) as f32 / 799_999.0;
//             Color::Srgba(Srgba {
//                 red: 1.0,
//                 green: 0.0,
//                 blue: 1.0 - intensity,
//                 alpha: 1.0,
//             })
//         }

//         // 3,200,001 to 3,999,600: Light Magenta to Magenta
//         3_200_001..=3_999_600 => {
//             let intensity = (value - 3_200_001) as f32 / 799_599.0;
//             Color::Srgba(Srgba {
//                 red: 1.0,
//                 green: 0.0,
//                 blue: 1.0 - intensity * 0.5,
//                 alpha: 1.0,
//             })
//         }

//         // 3,999,601+: White
//         _ => Color::Srgba(Srgba {
//             red: 1.0,
//             green: 1.0,
//             blue: 1.0,
//             alpha: 1.0,
//         }),
//     }
// }
pub fn get_bits_color(value: u64) -> Color {
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

pub fn get_leading_zeros_color(value: usize) -> Color {
    match value {
        // Black
        0..=29 => Color::Srgba(Srgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }),

        // BLUE
        30..=38 => {
            let intensity = (value - 8) as f32 / 3.0;
            Color::Srgba(Srgba {
                red: 0.0,
                green: 0.8 - intensity * 0.4,
                blue: 0.0,
                alpha: 1.0,
            })
        }

        // mEH
        39..=44 => {
            let intensity = (value - 12) as f32 / 3.0;
            Color::Srgba(Srgba {
                red: 0.0,
                green: 0.4 - intensity * 0.2,
                blue: intensity * 0.8,
                alpha: 1.0,
            })
        }

        // CYAN
        45..=60 => {
            let intensity = (value - 16) as f32 / 3.0;
            Color::Srgba(Srgba {
                red: 0.0,
                green: intensity * 0.5,
                blue: 1.0,
                alpha: 1.0,
            })
        }

        // RED
        61..=70 => {
            let intensity = (value - 20) as f32 / 3.0;
            Color::Srgba(Srgba {
                red: intensity,
                green: 0.0,
                blue: 1.0 - intensity * 0.5,
                alpha: 1.0,
            })
        }

        // 24–50: Transition Magenta → White
        71..=150 => {
            let range = (50 - 24) as f32;
            let intensity = (value - 24) as f32 / range;
            Color::Srgba(Srgba {
                red: 1.0,
                green: intensity,
                blue: 1.0,
                alpha: 1.0,
            })
        }

        // 51–256: Final range
        151..=256 => {
            let range = (256 - 51) as f32;
            let intensity = (value - 51) as f32 / range;
            Color::Srgba(Srgba {
                red: 1.0,
                green: 1.0,
                blue: 1.0 - intensity * 0.5,
                alpha: 1.0,
            })
        }

        // Above 256 (optional clamp)
        _ => Color::Srgba(Srgba {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        }),
    }
}

// pub fn get_leading_zeros_color(value: usize) -> Color {
//     match value {
//         // 0-7: Black (Very low difficulty)
//         0..=7 => Color::Srgba(Srgba {
//             red: 0.0,
//             green: 0.0,
//             blue: 0.0,
//             alpha: 1.0,
//         }),

//         // 8-11: Light Green to Green (Low difficulty)
//         8..=11 => {
//             let intensity = (value - 8) as f32 / 3.0;
//             Color::Srgba(Srgba {
//                 red: 0.0,
//                 green: 0.8 - intensity * 0.4,
//                 blue: 0.0,
//                 alpha: 1.0,
//             })
//         }

//         // 12-15: Green to Cyan (Moderate difficulty)
//         12..=15 => {
//             let intensity = (value - 12) as f32 / 3.0;
//             Color::Srgba(Srgba {
//                 red: 0.0,
//                 green: 0.4 - intensity * 0.2,
//                 blue: intensity * 0.8,
//                 alpha: 1.0,
//             })
//         }

//         // 16-19: Cyan to Blue (Increased difficulty)
//         16..=19 => {
//             let intensity = (value - 16) as f32 / 3.0;
//             Color::Srgba(Srgba {
//                 red: 0.0,
//                 green: intensity * 0.5,
//                 blue: 1.0,
//                 alpha: 1.0,
//             })
//         }

//         // 20-23: Blue to Magenta (High difficulty)
//         20..=23 => {
//             let intensity = (value - 20) as f32 / 3.0;
//             Color::Srgba(Srgba {
//                 red: intensity * 1.0,
//                 green: 0.0,
//                 blue: 1.0 - intensity * 0.5,
//                 alpha: 1.0,
//             })
//         }

//         // 24+: Magenta to White (Very high difficulty)
//         _ => {
//             let intensity = (value - 24) as f32 / 40.0;
//             Color::Srgba(Srgba {
//                 red: 1.0,
//                 green: intensity * 1.0,
//                 blue: 1.0,
//                 alpha: 1.0,
//             })
//         }
//     }
// }

pub fn get_excesswork_color(value: usize) -> Color {
    match value {
        // 1: Green
        0 => Color::Srgba(Srgba {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }),

        // 2: Light Blue
        1 => Color::Srgba(Srgba {
            red: 0.5,
            green: 0.5,
            blue: 1.0,
            alpha: 1.0,
        }),

        // 4: Yellow
        2 => Color::Srgba(Srgba {
            red: 1.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }),

        // 5: Orange
        3 => Color::Srgba(Srgba {
            red: 1.0,
            green: 0.5,
            blue: 0.0,
            alpha: 1.0,
        }),

        // 6: Red
        4 => Color::Srgba(Srgba {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }),

        // 7: Purplish Magenta
        5 => Color::Srgba(Srgba {
            red: 0.8,
            green: 0.0,
            blue: 0.8,
            alpha: 1.0,
        }),

        // 6+: Hot Pink (Intensifying with higher values)
        _ => {
            let intensity = ((value - 8) as f32 / 4.0).min(1.0);
            Color::Srgba(Srgba {
                red: 1.0,
                green: 0.0,
                blue: 0.5 + intensity * 0.5,
                alpha: 1.0,
            })
        }
    }
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
