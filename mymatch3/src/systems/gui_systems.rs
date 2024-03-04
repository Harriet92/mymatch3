use bevy::{
    prelude::*
};

use crate::components::score_components;
use crate::components::gui_components::ScoreboardUi;

pub fn update_scoreboard(scoreboard: Res<score_components::Scoreboard>, mut query: Query<&mut Text, With<ScoreboardUi>>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}