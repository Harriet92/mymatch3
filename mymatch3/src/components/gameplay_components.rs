use bevy::prelude::{Component, Event};

#[derive(Component)]
pub(crate) struct Tile {
    pub x: usize,
    pub y: usize,
    pub tile_type: usize
}

impl Tile {
    pub fn new(x: usize, y: usize, tile_type: usize) -> Tile {
        Tile { x, y, tile_type}
    }
}

#[derive(Component)]
pub struct NeedsView;

#[derive(Component)]
pub struct Selected;