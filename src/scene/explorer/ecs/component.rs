use bevy::prelude::Component;

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
pub struct Castle;
