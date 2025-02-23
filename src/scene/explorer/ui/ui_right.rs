use bevy::prelude::*;

use crate::scene::{
    explorer::ecs::component::{
        BlockTimeToggleBtn, ByteToggleBtn, ExcessWorkToggleBtn, FeeToggleBtn, LeadZerosToggleBtn,
        TgtDiffToggleBtn, TxCountToggleBtn, VersionToggleBtn, WeightToggleBtn,
    },
    initer::ecs::resource::UiColorPalette,
};

use super::{components::ExplorerUiNodeRight, toggle_button::spawn_game_toggle_button};

pub fn right_ui() {}
