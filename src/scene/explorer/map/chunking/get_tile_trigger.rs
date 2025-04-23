use bevy::prelude::*;
use chrono::{Duration, Utc};

use crate::{
    ecs::{resource::GameTimetamp, state::ExplorerCommsSubState},
    helper::plugins::browser::event::ReadGameTilesIdb,
    scene::explorer::ecs::state::InitSpawnMapState,
};
// #[default]
// Off,
// MapSpawn,
// LocalStorageRead,,
// Done,

pub fn init_local_storage_read(
    // mut get_tiles: EventWriter<RequestServerGameTiles>,
    mut browser_indexeddb: EventWriter<ReadGameTilesIdb>,
    //mut browser_writer: EventWriter<WriteGameTilesIdb>,
    mut gts: ResMut<GameTimetamp>,
    mut init_state: ResMut<NextState<InitSpawnMapState>>,
    mut comms_state: ResMut<NextState<ExplorerCommsSubState>>,
) {
    if gts.ts.is_some() {
        browser_indexeddb.write(ReadGameTilesIdb);
    } else {
        gts.ts = Some(Utc::now() - Duration::minutes(20));
        // get_tiles.write(RequestServerGameTiles);
        info!("sending height update pattern");
        comms_state.set(ExplorerCommsSubState::Live);
        init_state.set(InitSpawnMapState::Done);
    }
}
