use crate::ecs::state::SceneState;
use bevy::prelude::*;
use blockchain_key_ranges::BlockchainKeyRangesPlugin;
use toggle_button_q::{
    blocktime_btn, bytes_btn, excesswork_btn, fee_btn, leadzeros_btn, tgtdiff_btn,
    tgtdiff_diff_btn, txcount_btn, version_btn, weights_btn,
};

use self::{
    overall_ui::ui_explorer, ui_blockchain_toggles::left_ui, ui_bottom::bottom_ui,
    ui_right::right_ui, ui_top::top_ui,
};

use super::ecs::state::ExplorerSubState;

mod blockchain_key_ranges;
pub mod components;
pub mod ecs;
pub mod event;
mod overall_ui;
mod toggle_button;
mod toggle_button_q;
pub mod ui_blockchain_toggles;
pub mod ui_bottom;
pub mod ui_right;
mod ui_top;
pub struct ExplorerUiPlugin;

impl Plugin for ExplorerUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(SceneState::Explorer),
            (((
                (ui_explorer,),
                (top_ui, left_ui, right_ui, bottom_ui), //apply_deferred
            )
                .chain(),)
                .run_if(run_once),),
        )
        .add_systems(
            Update,
            (
                fee_btn,
                blocktime_btn,
                txcount_btn,
                bytes_btn,
                weights_btn,
                tgtdiff_btn,
                tgtdiff_diff_btn,
                leadzeros_btn,
                excesswork_btn,
                version_btn,
            )
                .run_if(in_state(ExplorerSubState::Running)),
        )
        .add_plugins(BlockchainKeyRangesPlugin);
    }
}
