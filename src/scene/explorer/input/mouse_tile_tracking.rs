use crate::scene::explorer::{
    ecs::component::SelectedTile,
    input::hard::RemoveTileManualSelectionSprite,
    map::ecs::component::{BaseTile, MainBaseTileMap, UlamComp},
};

use super::hard::{
    AddTileManualSelectionSprite, CursorPosInfo, CursorPosRaw, LastClickedTile,
    THRESHOLD_SELECT_MANUAL_CLICK,
};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn update_cursor_pos(
    //mouse: Res<ButtonInput<MouseButton>>,
    camera_q: Query<(&GlobalTransform, &Camera)>,
    //time: Res<Time>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_pos: ResMut<CursorPosRaw>,
) {
    for cursor_moved in cursor_moved_events.read() {
        // To get the mouse's world position, we have to transform its window position by
        // any transforms on the camera. This is done by projecting the cursor position into
        // camera space (world space).
        for (cam_t, cam) in camera_q.iter() {
            if let Ok(pos) = cam.viewport_to_world_2d(cam_t, cursor_moved.position) {
                *cursor_pos = CursorPosRaw(pos);
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn cursor_to_tile(
    cursor_pos: Res<CursorPosRaw>,
    tilemap_q: Query<
        (
            &TilemapSize,
            &TilemapGridSize,
            &TilemapTileSize,
            &TilemapType,
            &TileStorage,
            &Transform,
            &TilemapAnchor,
        ),
        With<MainBaseTileMap>,
    >,
    tile_q: Query<(&UlamComp, &SelectedTile), With<BaseTile>>,
    mut cursor_ulam_pos: ResMut<CursorPosInfo>,
) {
    for (map_size, grid_size, tile_size, map_type, tile_storage, map_transform, anchor) in
        tilemap_q.iter()
    {
        let cursor_pos: Vec2 = cursor_pos.0;
        let cursor_in_map_pos: Vec2 = {
            let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };
        // Once we have a world position we can transform it into a possible tile position.
        if let Some(tile_pos) = TilePos::from_world_pos(
            &cursor_in_map_pos,
            map_size,
            grid_size,
            tile_size,
            map_type,
            anchor,
        ) {
            // BACKUP LOGIC ULAM TRICK
            // cursor_ulam_pos.ulam = ulam::get_value_from_xy(
            //     tile_pos.x as i32 - (map_length.0 / 2) as i32,
            //     tile_pos.y as i32 - (map_length.0 / 2) as i32,
            // );

            if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                let (ulam_v, currently_selected) = tile_q.get(tile_entity).unwrap();
                cursor_ulam_pos.ent = tile_entity;
                cursor_ulam_pos.ulam = ulam_v.0;
                cursor_ulam_pos.currently_selected = currently_selected.0;
            }
        }
    }
}

pub fn attribute_click_on_map(
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut last_clicked_tile: ResMut<LastClickedTile>,
    cursor_pos: Res<CursorPosInfo>,
    mut send_add_manual_selection_event: EventWriter<AddTileManualSelectionSprite>,
    mut send_remove_manual_selection_event: EventWriter<RemoveTileManualSelectionSprite>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let now = time.elapsed_secs();
        let elapsed = now - last_clicked_tile.time;

        // info!(
        //     "selected: {:?}\nelap time: {}, thresh is: {}",
        //     cursor_pos, elapsed, THRESHOLD_SELECT_MANUAL_CLICK
        // );
        if cursor_pos.currently_selected {
            send_remove_manual_selection_event
                .write(RemoveTileManualSelectionSprite(cursor_pos.ent));
            info!(
                "we have a click on existing selection - removing {}!",
                cursor_pos.ulam
            );
        } else if last_clicked_tile.ulam == cursor_pos.ulam
            && elapsed < THRESHOLD_SELECT_MANUAL_CLICK
        {
            send_add_manual_selection_event.write(AddTileManualSelectionSprite(cursor_pos.ent));
            info!(
                "we have a second click on a tile - adding {}!",
                cursor_pos.ulam
            );
        }
        last_clicked_tile.ulam = cursor_pos.ulam;
        last_clicked_tile.time = now;
    }
}
