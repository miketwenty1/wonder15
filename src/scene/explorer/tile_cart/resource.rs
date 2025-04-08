use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Clone, Debug, Default)]
pub struct UserPurchaseHistory {
    pub username: String,
    pub last_paid_amount: u32,
    pub message: String,
    pub color: Color,
}

#[derive(Resource, Clone, Debug, Default)]
pub struct TileCartItem {
    pub messages: Option<Vec<UserPurchaseHistory>>,
    pub cost: u32,
    pub height: u32,
    pub new_ln_address: String,
    pub new_username: String,
    pub new_color: Color,
    pub new_color_text: String,
    pub new_message: String,
}

#[derive(Resource, Clone, Default)]
pub struct TileCart {
    pub map: HashMap<u32, TileCartItem>,
}

#[derive(Resource, Clone, Debug, Default)]
pub struct TileCartVec {
    pub vec: Vec<TileCartItem>,
    pub index: usize,
}
