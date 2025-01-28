use bevy::{
    prelude::{Component, Deref, DerefMut},
    time::Timer,
};

#[derive(Component)]
pub struct AnimationIndicesComp {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimerComp(pub Timer);
