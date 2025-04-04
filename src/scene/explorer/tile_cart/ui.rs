use bevy::{prelude::*, text::FontSmoothing};

use crate::scene::{
    explorer::{
        blockchain_ui::components::{ExplorerUiNodeBottom, ExplorerUiNodeTop},
        ecs::{
            component::GeneralUiBtn,
            hard::{UI_MEDIUM_TEXT_SIZE, UI_SMALL_TEXT_SIZE},
        },
    },
    initer::ecs::resource::UiColorPalette,
};

use super::{
    component::{
        BlockCountText, CancelExplorerSelectionCartBtn, CartPriceText, InspectOrBuyExporerCartBtn,
        TileCartParentNode,
    },
    event::RefreshTileCart,
    state::ExplorerRunningCartSub2State,
};

pub fn spawn_selection_info(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    colors: Res<UiColorPalette>,
    placement_query: Query<Entity, With<ExplorerUiNodeTop>>,
    mut refresh: EventWriter<RefreshTileCart>,
) {
    for parent_node in placement_query.iter() {
        info!("layout for amount??");
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        commands
            .spawn((
                StateScoped(ExplorerRunningCartSub2State::On),
                Node {
                    padding: UiRect {
                        left: Val::Px(10.0),
                        right: Val::Px(10.0),
                        top: Val::Px(4.0),
                        bottom: Val::Px(4.0),
                    },
                    display: Display::Flex,
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                TileCartParentNode,
                BackgroundColor(colors.button_color),
                BorderRadius::all(Val::Px(8.0)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    //Blocks Selected: 0
                    Text::new(""),
                    TextFont {
                        font: font.clone(),
                        font_size: UI_MEDIUM_TEXT_SIZE,
                        font_smoothing: FontSmoothing::AntiAliased,
                    },
                    TextColor(colors.text_color),
                    BlockCountText,
                ));
            })
            .set_parent(parent_node);

        let _block_count_node = commands
            .spawn((
                StateScoped(ExplorerRunningCartSub2State::On),
                Node {
                    padding: UiRect {
                        left: Val::Px(10.0),
                        right: Val::Px(10.0),
                        top: Val::Px(4.0),
                        bottom: Val::Px(4.0),
                    },
                    display: Display::Flex,
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                BorderRadius::all(Val::Px(8.0)),
                BackgroundColor(colors.button_color),
            ))
            .with_children(|parent| {
                parent.spawn((
                    //Price: 0 satoshis
                    Text::new(""),
                    TextFont {
                        font,
                        font_size: UI_MEDIUM_TEXT_SIZE,
                        font_smoothing: FontSmoothing::AntiAliased,
                    },
                    TextColor(colors.text_color),
                    CartPriceText,
                ));
            })
            .set_parent(parent_node);
        refresh.send(RefreshTileCart);
    }
}

pub fn spawn_explorer_buttons_for_tilecart(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    colors: Res<UiColorPalette>,
    placement_query: Query<Entity, With<ExplorerUiNodeBottom>>,
) {
    for parent_node in placement_query.iter() {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        commands
            .spawn((
                StateScoped(ExplorerRunningCartSub2State::On),
                Button,
                Node {
                    width: Val::Px(120.),
                    height: Val::Px(40.),
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                },
                GeneralUiBtn,
                CancelExplorerSelectionCartBtn,
                BackgroundColor(colors.button_color),
                BorderColor(colors.node_color),
                BorderRadius::all(Val::Px(6.0)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Cancel Selection"),
                    TextFont {
                        font: font.clone(),
                        font_size: UI_SMALL_TEXT_SIZE,
                        font_smoothing: FontSmoothing::AntiAliased,
                    },
                    TextColor(colors.text_color),
                ));
            })
            .set_parent(parent_node);

        commands
            .spawn((
                StateScoped(ExplorerRunningCartSub2State::On),
                Button,
                Node {
                    width: Val::Px(120.),
                    height: Val::Px(40.),
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                },
                InspectOrBuyExporerCartBtn,
                GeneralUiBtn,
                BackgroundColor(colors.button_color),
                BorderColor(colors.node_color),
                BorderRadius::all(Val::Px(6.0)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Inspect or Buy"),
                    TextFont {
                        font: font.clone(),
                        font_size: UI_SMALL_TEXT_SIZE,
                        font_smoothing: FontSmoothing::AntiAliased,
                    },
                    TextColor(colors.text_color),
                ));
            })
            .set_parent(parent_node);
    }
}
