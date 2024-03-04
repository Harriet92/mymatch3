use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Scoreboard {
    pub score: usize,
}
