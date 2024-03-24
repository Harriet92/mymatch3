use bevy::prelude::{Component, Deref, DerefMut, Entity, Event, Timer};

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct MainCamera;

#[derive(Event)]
pub struct TileClickedEvent {
    pub x: usize,
    pub y: usize
}