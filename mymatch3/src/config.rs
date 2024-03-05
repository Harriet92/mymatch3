use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GameplayConfig {
    pub board_size: usize,
    pub increment: usize
}

impl GameplayConfig {
    pub fn default() -> GameplayConfig {
        GameplayConfig {
            board_size: 10,
            increment: 1,
        }
    }
}
