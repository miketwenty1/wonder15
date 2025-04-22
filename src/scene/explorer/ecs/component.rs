use bevy::prelude::*;

#[derive(Component)]
pub struct BlockchainFilterToggleParent;

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

#[derive(Component)]
pub struct SelectedTile(pub bool);

#[derive(Component)]
pub struct GeneralUiBtn;

#[derive(Component)]
pub struct RunningHal;

#[derive(Component)]
pub struct HalTargetBlock(pub u32);

#[derive(Component, Deref, DerefMut)]
pub struct HalTargetXY(pub Vec2);

#[derive(Component)]
pub struct HalSpeed(pub f32);

#[derive(Component)]
pub struct HalThere(pub bool);

#[derive(Component)]
pub struct HalPower(pub u32);

#[derive(Component)]
pub struct HomeTile(pub u32);

#[derive(Component)]
pub struct BuildingTileComp;

#[derive(Component)]
pub struct Castle;
