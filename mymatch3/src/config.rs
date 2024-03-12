use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GameplayConfig {
    pub board_size: usize,
    pub increment: usize,
    pub tile_types_count: usize
}

impl GameplayConfig {
    pub fn default() -> GameplayConfig {
        GameplayConfig {
            board_size: 8,
            increment: 1,
            tile_types_count: 7
        }
    }
}


