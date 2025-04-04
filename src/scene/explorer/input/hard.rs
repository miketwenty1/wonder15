use bevy::prelude::*;

pub const MOVE_VELOCITY: f32 = 100.;
pub const THRESHOLD_SELECT_MANUAL_CLICK: f32 = 2.5;
#[derive(Resource, DerefMut, Deref)]
pub struct CursorPosRaw(pub Vec2);
// {
//     // pub ulam: u32,
//     pub xy: Vec2,
// }

impl Default for CursorPosRaw {
    fn default() -> Self {
        // Self {
        //     // ulam: 999_999_999,
        //     xy: Vec2::new(-100_000.0, -100_000.0),
        // }
        Self(Vec2::new(-100_000.0, -100_000.0))
    }
}

#[derive(Debug, Resource)]
pub struct CursorPosInfo {
    pub ulam: u32,
    pub ent: Entity,
    pub currently_selected: bool,
}

impl Default for CursorPosInfo {
    fn default() -> Self {
        Self {
            ulam: 999_999_999,
            ent: Entity::PLACEHOLDER,
            currently_selected: false,
        }
    }
}

#[derive(Debug, Resource)]
pub struct LastClickedTile {
    pub ulam: u32,
    pub time: f32,
}
impl Default for LastClickedTile {
    fn default() -> Self {
        Self {
            ulam: 999_999_999,
            time: 0.,
        }
    }
}

#[derive(Event, Debug)]
pub struct AddTileManualSelectionSprite(pub Entity);

#[derive(Event, Debug)]
pub struct RemoveTileManualSelectionSprite(pub Entity);
