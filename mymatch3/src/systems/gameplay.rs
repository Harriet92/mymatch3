use bevy::{
    prelude::*
};

use crate::components::score_components;

pub fn test_gameplay(mut scoreboard: ResMut<score_components::Scoreboard>) {
    scoreboard.score += 1;
}