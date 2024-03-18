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

#[derive(Resource)]
pub struct GameplayViewConfig {
    pub sprite_scale: f32,
    pub sprite_size: f32,
    pub tile_size: f32,
    pub translation: f32
}

impl GameplayViewConfig {
    pub fn default(board_size: usize) -> GameplayViewConfig {
        let sprite_size = 512.;
        let sprite_scale = 0.15;
        let tile_size = sprite_size * sprite_scale;
        let translation = tile_size * board_size as f32 / 2. - tile_size / 2.;

        GameplayViewConfig {
            sprite_size,
            sprite_scale,
            tile_size,
            translation
        }
    }
}


