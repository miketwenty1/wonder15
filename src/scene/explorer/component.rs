use bevy::{
    prelude::{Component, Deref, DerefMut},
    time::Timer,
};

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
