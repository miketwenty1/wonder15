use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    text::FontSmoothing,
};

use crate::scene::{
    explorer::ui::{components::ExplorerUiNodeRight, ecs::state::ColorBlockchainKeySubState},
    initer::ecs::resource::{BlockchainKeyValues, UiColorPalette},
};

#[allow(clippy::too_many_arguments)]
pub fn spawn_fee_legend(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    placement_query: Query<Entity, With<ExplorerUiNodeRight>>,
    ui_colors: Res<UiColorPalette>,
    key_color_ranges: Res<BlockchainKeyValues>,
    mut images: ResMut<Assets<Image>>,
    //blockchain_legend_values: Res<BlockchainKeyValues>,
) {
    for ent in placement_query.iter() {
        info!("spawning right side ui with fee keys");
        let mut side_parent = commands.spawn((
            Node {
                justify_items: JustifyItems::Start,
                justify_self: JustifySelf::Start,
                flex_direction: FlexDirection::Column,
                width: Val::Px(200.),
                // padding: UiRect::all(Val::Px(4.0)),
                margin: UiRect::bottom(Val::Auto),
                ..default()
            },
            BackgroundColor(ui_colors.node_color),
            StateScoped(ColorBlockchainKeySubState::Fee),
            BorderRadius::all(Val::Px(5.0)),
        ));

        side_parent
            .with_child((
                Node {
                    display: Display::Flex,
                    margin: UiRect::horizontal(Val::Px(6.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BorderColor(Color::WHITE),
            ))
            .with_children(|inner| {
                let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                inner.spawn((
                    Text::new(format!("Fees per block")),
                    TextFont {
                        font: font.clone(),
                        font_size: 15.,
                        font_smoothing: FontSmoothing::AntiAliased,
                    },
                    TextColor(ui_colors.text_color),
                ));
            });

        let end_text = key_color_ranges.fee.vec.last().unwrap().end.0;
        info!("end_text: {}", end_text);
        for range in &key_color_ranges.fee.vec {
            side_parent.with_children(|parent| {
                let mut row: EntityCommands<'_> = parent.spawn((Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    row_gap: Val::Px(4.0),
                    padding: UiRect::all(Val::Px(6.0)),
                    flex_wrap: FlexWrap::NoWrap,
                    //margin: UiRect::all(Val::Px(2.0)),
                    ..default()
                },));
                row.with_children(|builder| {
                    let left = range.start.1;
                    let right = range.end.1;

                    let first_val_range = format_sats(range.start.0);
                    info!("F {} -> {}", range.start.0, first_val_range);
                    let (dash, second_val_range) = if range.end.0 == end_text {
                        ("", "+")
                    } else {
                        info!("L {} -> {}", range.end.0, &format_sats(range.end.0));
                        (" - ", &format_sats(range.end.0) as &str)
                    };

                    let image_handle = make_gradient_image(&mut images, 15, 6, left, right);
                    builder.spawn((
                        ImageNode {
                            image: image_handle,
                            ..default()
                        },
                        Node {
                            margin: UiRect::right(Val::Px(10.)),
                            display: Display::Flex,
                            ..default()
                        },
                    ));

                    builder
                        .spawn(Node {
                            display: Display::Flex,
                            margin: UiRect::horizontal(Val::Px(6.0)),
                            ..default()
                        })
                        .with_children(|inner| {
                            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                            inner.spawn((
                                Text::new(first_val_range),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 15.,
                                    font_smoothing: FontSmoothing::AntiAliased,
                                },
                                TextColor(ui_colors.text_color),
                            ));
                        });
                    builder
                        .spawn(Node {
                            display: Display::Flex,
                            ..default()
                        })
                        .with_children(|inner| {
                            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                            inner.spawn((
                                Text::new(dash),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 15.,
                                    font_smoothing: FontSmoothing::AntiAliased,
                                },
                                TextColor(ui_colors.text_color),
                            ));
                        });

                    builder
                        .spawn(Node {
                            display: Display::Flex,
                            ..default()
                        })
                        .with_children(|inner| {
                            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                            inner.spawn((
                                Text::new(second_val_range),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 15.,
                                    font_smoothing: FontSmoothing::AntiAliased,
                                },
                                TextColor(ui_colors.text_color),
                            ));
                        });
                });
            });
        }

        side_parent.set_parent(ent);
    }
}

fn make_gradient_image(
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
            let c = lerp_color(color_left, color_right, t);
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

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let a = a.to_linear();
    let b = b.to_linear();
    Color::srgba(
        a.red + t * (b.red - a.red),
        a.green + t * (b.green - a.green),
        a.blue + t * (b.blue - a.blue),
        a.alpha + t * (b.alpha - a.alpha),
    )
}

fn format_sats<T: Into<u64>>(value: T) -> String {
    let n = value.into();

    match n {
        0..=9_999 => format!("{} sats", n),
        10_000..=999_999 => {
            let k = n / 1_000;
            let r = n % 1_000;
            if r == 0 {
                format!("{}K sats", k)
            } else {
                format!("{:.1}K sats", n as f64 / 1_000.0)
            }
        }
        1_000_000..=99_999_999 => {
            let m = n / 1_000_000;
            let r = n % 1_000_000;
            if r == 0 {
                format!("{}M sats", m)
            } else {
                format!("{:.1}M sats", n as f64 / 1_000_000.0)
            }
        }
        100_000_000..=999_999_999 => {
            let whole_btc = n / 100_000_000;
            let r = n % 100_000_000;
            if r == 0 {
                format!("{} BTC", whole_btc)
            } else {
                format!("{:.1} BTC", n as f64 / 100_000_000.0)
            }
        }
        _ => {
            // n >= 1,000,000,000 sats (>= 10 BTC), no decimal
            format!("{} BTC", n / 100_000_000)
        }
    }
}
