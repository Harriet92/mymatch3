use bevy::prelude::Component;

#[derive(Component)]
pub(crate) struct Tile {
    pub x: usize,
    pub y: usize,
}
