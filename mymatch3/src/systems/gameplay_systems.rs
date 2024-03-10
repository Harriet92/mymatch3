use bevy::{
    prelude::*
};

use crate::components::score_components;
use crate::config::GameplayConfig;

pub fn test_gameplay(mut scoreboard: ResMut<score_components::Scoreboard>,
gameplay_config: Res<GameplayConfig>) {
    scoreboard.score += gameplay_config.increment;
}

pub fn spawn_board(mut commands: Commands, gameplay_config: Res<GameplayConfig>){
    for x in 0..gameplay_config.board_size {
        for y in 0..gameplay_config.board_size {
            //spawn_tile(commands, x, y);
        }
    }
}

fn spawn_tile(mut commands: Commands, x: usize, y: usize) {
    todo!()
}