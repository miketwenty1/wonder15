use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TileColor, TileTextureIndex};
use rand::Rng;

use crate::{
    ecs::resource::WorldBlockchainTileMap,
    helper::utils::funs::{
        bits_to_target_hash, get_text_color_per_tile_color, leading_zeros_in_32,
        percent_change_of_difficulties,
    },
    scene::{
        explorer::{
            ecs::{event::SwapTilesEvent, resource::CurrentTilesRes},
            map::{
                ecs::{
                    component::{
                        AssociatedTileColor, LandIndexComp, PlayerTileColorComp, TileText, UlamComp,
                    },
                    hard::TEXTURE_INDEX_FOR_PLAYER_COLOR,
                },
                tile_manipulation::blockchain_color::{get_bits_color, get_version_color},
            },
        },
        initer::ecs::resource::BlockchainFilterKeys,
    },
};

pub const WEIRD_COLOR: Color = Color::Srgba(Srgba {
    red: 0.3,
    green: 0.3,
    blue: 0.3,
    alpha: 1.,
});

pub fn swap_tile_index_reader(
    mut event_r: EventReader<SwapTilesEvent>,
    mut query: Query<(
        &mut TileTextureIndex,
        &LandIndexComp,
        &mut TileColor,
        &PlayerTileColorComp,
        &UlamComp,
    )>,
    mut q_text_color: Query<(&mut TextColor, &AssociatedTileColor), With<TileText>>,
    mut current_tiles: ResMut<CurrentTilesRes>,
    blockchain_tiles: Res<WorldBlockchainTileMap>,
    blockchain_legend_colors: Res<BlockchainFilterKeys>,
) {
    for e in event_r.read() {
        let swap_type = if e == &SwapTilesEvent::Iter {
            current_tiles.0.next_tile_swap()
        } else {
            e.clone()
        };

        match swap_type {
            SwapTilesEvent::PlayerColor => {
                for (mut tile_index, _, mut tile_color, player_tile_color, _) in query.iter_mut() {
                    if *tile_index != TileTextureIndex(TEXTURE_INDEX_FOR_PLAYER_COLOR) {
                        *tile_color = player_tile_color.0;
                        *tile_index = TileTextureIndex(TEXTURE_INDEX_FOR_PLAYER_COLOR);
                    }
                }
                for (mut text_color, player_color) in q_text_color.iter_mut() {
                    *text_color = TextColor(get_text_color_per_tile_color(&player_color.0));
                }
            }
            SwapTilesEvent::Land => {
                for (mut tile_index, land_index, mut tile_color, _, _) in query.iter_mut() {
                    if *tile_index == TileTextureIndex(TEXTURE_INDEX_FOR_PLAYER_COLOR) {
                        *tile_color = TileColor(Color::Srgba(Color::WHITE.into()));
                        *tile_index = TileTextureIndex(land_index.0);
                    }
                }
                for (mut text_color, _) in q_text_color.iter_mut() {
                    *text_color = TextColor(Color::WHITE);
                }
            }
            SwapTilesEvent::Fee => {
                info!("Fee");
                let filter_legend = blockchain_legend_colors.fee.clone();

                for (mut tile_index, _, mut tile_color, _, ulam) in query.iter_mut() {
                    if let Some(val) = blockchain_tiles.map.get(&ulam.0) {
                        let c = match filter_legend
                            .color_for_ranges(val.block_fee.try_into().unwrap())
                        {
                            Some(s) => s,
                            None => filter_legend.vec.last().unwrap().end.1.into(),
                        };
                        tile_color.0 = Color::Srgba(c);
                    } else {
                        tile_color.0 = WEIRD_COLOR;
                    }

                    *tile_index = TileTextureIndex(TEXTURE_INDEX_FOR_PLAYER_COLOR);
                }
            }
            SwapTilesEvent::BlockTime => {
                info!("BlockTime");
                let filter_legend = blockchain_legend_colors.block_time.clone();
                for (mut tile_index, _, mut tile_color, _, ulam) in query.iter_mut() {
                    if let Some(val) = blockchain_tiles.map.get(&ulam.0) {
                        let current = val.block_time;
                        let prev_o = blockchain_tiles.map.get(&(ulam.0 - 1));
                        let previous = prev_o.map_or(0, |prev| prev.block_time);
                        let c = match filter_legend
                            .color_for_ranges(current as i64 - previous as i64)
                        {
                            Some(s) => s,
                            None => filter_legend.vec.last().unwrap().end.1.into(),
                        };
                        tile_color.0 = Color::Srgba(c);
                    } else {
                        tile_color.0 = WEIRD_COLOR;
                    }

                    *tile_index = TileTextureIndex(TEXTURE_INDEX_FOR_PLAYER_COLOR);
                }
            }
            SwapTilesEvent::TxCount => {
                info!("TxCount");
                let filter_legend = blockchain_legend_colors.tx_count.clone();
                for (mut tile_index, _, mut tile_color, _, ulam) in query.iter_mut() {
                    if let Some(val) = blockchain_tiles.map.get(&ulam.0) {
                        let c = match filter_legend.color_for_ranges(val.block_n_tx.into()) {
                            Some(s) => s,
                            None => filter_legend.vec.last().unwrap().end.1.into(),
                        };
                        if val.height == 170 || val.height == 167 {
                            info!("{} {} {:?}", val.height, val.block_n_tx, c);
                        }

                        //tile_color.0 = c;
                        tile_color.0 = Color::Srgba(c);
                    } else {
                        tile_color.0 = WEIRD_COLOR;
                    }

                    *tile_index = TileTextureIndex(TEXTURE_INDEX_FOR_PLAYER_COLOR);
                }
            }
            SwapTilesEvent::Byte => {
                info!("Byte");
                let filter_legend = blockchain_legend_colors.byte.clone();
                for (mut tile_index, _, mut tile_color, _, ulam) in query.iter_mut() {
                    if let Some(val) = blockchain_tiles.map.get(&ulam.0) {
                        // let c = get_byte_color(val.block_size);
                        // tile_color.0 = c;

                        let c = match filter_legend.color_for_ranges(val.block_size.into()) {
                            Some(s) => s,
                            None => filter_legend.vec.last().unwrap().end.1.into(),
                        };
                        //tile_color.0 = c;
                        tile_color.0 = Color::Srgba(c);
                    } else {
                        tile_color.0 = WEIRD_COLOR;
                    }

                    *tile_index = TileTextureIndex(TEXTURE_INDEX_FOR_PLAYER_COLOR);
                }
            }
            SwapTilesEvent::Weight => {
                info!("Weight");
                let filter_legend = blockchain_legend_colors.weight.clone();
                for (mut tile_index, _, mut tile_color, _, ulam) in query.iter_mut() {
                    if let Some(val) = blockchain_tiles.map.get(&ulam.0) {
                        let c = match filter_legend
                            .color_for_ranges(val.block_weight.try_into().unwrap())
                        {
                            Some(s) => s,
                            None => filter_legend.vec.last().unwrap().end.1.into(),
                        };
                        tile_color.0 = Color::Srgba(c);
                    } else {
                        tile_color.0 = WEIRD_COLOR;
                    }

                    *tile_index = TileTextureIndex(TEXTURE_INDEX_FOR_PLAYER_COLOR);
                }
            }
            SwapTilesEvent::TargetDifficulty => {
                info!("TargetDifficulty");
                for (mut tile_index, _, mut tile_color, _, ulam) in query.iter_mut() {
                    if let Some(val) = blockchain_tiles.map.get(&ulam.0) {
                        let c = get_bits_color(val.block_bits);
                        tile_color.0 = c;
                    } else {
                        tile_color.0 = WEIRD_COLOR;
                    }

                    *tile_index = TileTextureIndex(TEXTURE_INDEX_FOR_PLAYER_COLOR);
                }
            }
            SwapTilesEvent::TargetDifficultyDiff => {
                info!("TargetDifficultyDiff");
                let filter_legend = blockchain_legend_colors.tgt_diff_diff.clone();
                for (mut tile_index, _, mut tile_color, _, ulam) in query.iter_mut() {
                    if let Some(val) = blockchain_tiles.map.get(&ulam.0) {
                        let mut current = val.block_bits;
                        let prev_o = blockchain_tiles.map.get(&(ulam.0 - 2016));
                        let mut previous = prev_o.map_or(val.block_bits, |prev| prev.block_bits);

                        // let mut rng = rand::thread_rng();
                        // let current = rng.gen_range(100.0..400.0);
                        // let previous = rng.gen_range(100.0..400.0);
                        let percent_change = percent_change_of_difficulties(previous, current);

                        let c = match filter_legend.color_for_ranges(percent_change as i64) {
                            Some(s) => s,
                            None => filter_legend.vec.last().unwrap().end.1.into(),
                        };

                        if val.height == 689472 {
                            info!("{} % change {}", val.height, percent_change);
                        }
                        if percent_change > 300 {
                            info!("{} % change {}", val.height, percent_change);
                        }
                        //info!("percent change {}, color {:?}", percent_change, c);
                        tile_color.0 = Color::Srgba(c);
                    } else {
                        tile_color.0 = WEIRD_COLOR;
                    }

                    *tile_index = TileTextureIndex(TEXTURE_INDEX_FOR_PLAYER_COLOR);
                }
            }
            SwapTilesEvent::LeadingZeros => {
                info!("LeadingZeros");
                let filter_legend = blockchain_legend_colors.leading_zeros.clone();
                for (mut tile_index, _, mut tile_color, _, ulam) in query.iter_mut() {
                    if let Some(val) = blockchain_tiles.map.get(&ulam.0) {
                        let hash = val.block_hash;
                        let leading_zeros = leading_zeros_in_32(&hash);

                        if val.height > 531766 && val.height < 531769 {
                            info!("{:?}", val);
                            info!("leading zeros for {} {}", val.height, leading_zeros);
                        }

                        let c = match filter_legend.color_for_ranges(leading_zeros as i64) {
                            Some(s) => s,
                            None => filter_legend.vec.last().unwrap().end.1.into(),
                        };
                        tile_color.0 = Color::Srgba(c);
                    } else {
                        tile_color.0 = WEIRD_COLOR;
                    }

                    *tile_index = TileTextureIndex(TEXTURE_INDEX_FOR_PLAYER_COLOR);
                }
            }
            SwapTilesEvent::ExcessWork => {
                let filter_legend = blockchain_legend_colors.excess_work.clone();
                info!("ExcessWork");
                for (mut tile_index, _, mut tile_color, _, ulam) in query.iter_mut() {
                    if let Some(val) = blockchain_tiles.map.get(&ulam.0) {
                        let hash = val.block_hash;
                        let leading_zeros = leading_zeros_in_32(&hash);
                        let target_hash = bits_to_target_hash(val.block_bits);
                        let diff = leading_zeros - target_hash;
                        let c = match filter_legend.color_for_ranges(diff as i64) {
                            Some(s) => s,
                            None => filter_legend.vec.last().unwrap().end.1.into(),
                        };
                        if diff > 14 {
                            info!("zz {} - {}", val.height, diff);
                        }
                        tile_color.0 = Color::Srgba(c);
                    } else {
                        tile_color.0 = WEIRD_COLOR;
                    }

                    *tile_index = TileTextureIndex(TEXTURE_INDEX_FOR_PLAYER_COLOR);
                }
            }
            SwapTilesEvent::Version => {
                let filter_legend = blockchain_legend_colors.version.clone();
                info!("Version");
                for (mut tile_index, _, mut tile_color, _, ulam) in query.iter_mut() {
                    if let Some(val) = blockchain_tiles.map.get(&ulam.0) {
                        let c = match filter_legend
                            .color_for_ranges(val.block_ver.try_into().unwrap())
                        {
                            Some(s) => s,
                            None => get_version_color(val.block_ver).into(),
                        };
                        tile_color.0 = Color::Srgba(c);
                    } else {
                        tile_color.0 = WEIRD_COLOR;
                    }

                    *tile_index = TileTextureIndex(TEXTURE_INDEX_FOR_PLAYER_COLOR);
                }
            }
            SwapTilesEvent::Iter => {
                info!("this shouldnt be reached this was taken care of above for swap_type");
            }
        }
        *current_tiles = CurrentTilesRes(e.clone());
    }
}
