use bevy::prelude::*;
use ulam::Quad;

use crate::scene::explorer::ecs::hard::BUILDING_Z;

const RADIAN_90: f32 = 1.5707961;
#[allow(clippy::too_many_arguments)]
pub fn spawn_road(
    texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    builder: &mut ChildBuilder,
    color: Color,
    offset: usize,
    height: u32,
) {
    // index number on sprite sheet and rotation degree. seems like 90 degrees is = 1.5707961
    let quad = ulam::quad_of_value(height);

    // because i want the roads to connect nicely on the bottom right corner,
    // I need to adjust which type of road angle I spawn
    let game_quad = if height == 1 {
        Quad::SouthEast
    } else if quad == Quad::SouthEast {
        Quad::South
    } else if quad == Quad::East && ulam::quad_of_value(height - 1) == Quad::SouthEast {
        Quad::SouthEast
    } else {
        quad
    };

    let road: (usize, f32) = match game_quad {
        ulam::Quad::North => (2, 0.0),
        ulam::Quad::NorthEast => (3, RADIAN_90),
        ulam::Quad::East => (2, RADIAN_90),
        ulam::Quad::SouthEast => (3, 0.0),
        ulam::Quad::South => (2, 0.0),
        ulam::Quad::SouthWest => (3, RADIAN_90 * 3.),
        ulam::Quad::West => (2, RADIAN_90),
        ulam::Quad::NorthWest => (3, RADIAN_90 * 2.),
        ulam::Quad::Center => (2, 0.0),
    };
    let transform = Transform {
        translation: Vec3::new(0., 0., BUILDING_Z - 0.1),
        scale: Vec3::new(3., 3., 1.0),
        rotation: Quat::from_rotation_z(road.1),
    };
    builder.spawn((
        Sprite {
            color,
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layout.clone(),
                index: road.0 + offset,
            }),
            ..Default::default()
        },
        transform,
        //BuildingStructure::Road,
    ));
}
