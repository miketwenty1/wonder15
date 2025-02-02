use bevy::ecs::system::Resource;
use serde::Deserialize;

#[derive(Resource, Clone)]
pub struct GameStaticInputs {
    pub username: String,
    pub ln_address: String,
    pub using_iphone: bool,
    pub server_url: String,
    pub blockchain_filters: bool,
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct BlockchainHeight(pub u32);

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct WinSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct GameHeight(pub u32);
