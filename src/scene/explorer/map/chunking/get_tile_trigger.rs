use bevy::prelude::*;

use crate::{
    ecs::state::ExplorerCommsSubState,
    helper::plugins::comms::ecs::{event::GetTileUpdates, structy::GetTileType},
};

pub fn after_map_init(
    mut comm_map_state: ResMut<NextState<ExplorerCommsSubState>>,
    mut get_tiles: EventWriter<GetTileUpdates>,
) {
    comm_map_state.set(ExplorerCommsSubState::Height);
    // if data already is loaded from local then TS otherwise Height.
    get_tiles.send(GetTileUpdates(GetTileType::Height));
    info!("sending height want");
}
