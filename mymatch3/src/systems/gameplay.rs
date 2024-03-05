use bevy::{
    prelude::*
};

use crate::components::score_components;
use crate::config::GameplayConfig;

pub fn test_gameplay(mut scoreboard: ResMut<score_components::Scoreboard>,
gameplay_config: Res<GameplayConfig>) {
    scoreboard.score += gameplay_config.increment;
}