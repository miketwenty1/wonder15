use bevy::{
    prelude::{Component, Deref, DerefMut},
    time::Timer,
};

#[derive(Component)]
pub struct FeeToggleBtn;

#[derive(Component)]
pub struct BlockTimeToggleBtn;

#[derive(Component)]
pub struct TxCountToggleBtn;

#[derive(Component)]
pub struct ByteToggleBtn;

#[derive(Component)]
pub struct WeightToggleBtn;
#[derive(Component)]
pub struct TgtDiffToggleBtn;

#[derive(Component)]
pub struct TgtDiffDiffToggleBtn;

#[derive(Component)]
pub struct LeadZerosToggleBtn;

#[derive(Component)]
pub struct ExcessWorkToggleBtn;

#[derive(Component)]
pub struct VersionToggleBtn;
