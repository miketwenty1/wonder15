use bevy::{prelude::*, text::FontSmoothing};

use super::components::{AmountNode, AmountText, BlockCountNode, ExplorerUiNodeTop};

pub fn top_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    placement_query: Query<Entity, With<ExplorerUiNodeTop>>,
) {
    for parent_node in placement_query.iter() {}
}
