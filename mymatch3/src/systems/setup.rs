use bevy::{
    prelude::*,
    ecs::system::Commands
};

use crate::components::*;

const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);

pub fn spawn_ui_system(
    mut commands: Commands
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Scoreboard
    commands.spawn((
        gui_components::ScoreboardUi,
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ]).with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
    ));
}