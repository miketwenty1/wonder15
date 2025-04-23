use bevy::ecs::event::Event;
use serde_json::Value;

// #[derive(Event, Debug)]
// pub struct RequestServerGameTiles;

// #[derive(Event, Debug)]
// pub struct GetBlockchainD(pub u32);

#[derive(Event)]
pub struct BrowserBlockchainDataEvent(pub Value);
#[derive(Event)]
pub struct BrowserGameTilesEvent(pub Value);
#[derive(Event)]
pub struct ServerBlockchainDataEvent(pub Value);
#[derive(Event)]
pub struct GameTilesByTsEvent(pub Value);
#[derive(Event)]
pub struct GameTilesByHeightEvent(pub Value);
#[derive(Event)]
pub struct CheckLnPaymentEvent(pub Value);
#[derive(Event)]
pub struct LnInvoiceEvent(pub Value);
#[derive(Event)]
pub struct UserInventoryEvent(pub Value);
#[derive(Event)]
pub struct BlockDataEvent(pub Value);
