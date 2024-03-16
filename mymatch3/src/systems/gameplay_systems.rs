use bevy::{
    prelude::*
};
use rand::{Rng, thread_rng};

use crate::components::{input_components, score_components};
use crate::components::gameplay_components;
use crate::components::gameplay_components::{LeftMouseButtonPressed, Tile};
use crate::config::GameplayConfig;


pub fn test_gameplay(mut scoreboard: ResMut<score_components::Scoreboard>,
gameplay_config: Res<GameplayConfig>) {
    scoreboard.score += gameplay_config.increment;
}

pub fn spawn_board(mut commands: Commands, gameplay_config: Res<GameplayConfig>){
    for x in 0..gameplay_config.board_size {
        for y in 0..gameplay_config.board_size {
            let random_tile_type = thread_rng().gen_range(0..gameplay_config.tile_types_count);
            spawn_tile(&mut commands, x, y, random_tile_type);
        }
    }
}

fn spawn_tile(mut commands: &mut Commands, x: usize, y: usize, tile_type: usize) {
    commands.spawn((
                       gameplay_components::Tile::new( x, y, tile_type),
                        gameplay_components::NeedsView
                   ));
}

pub fn mark_clicked_tile(cursor_position: Res<input_components::CurrentWorldCoords>,
    mut ev_clicked: EventReader<LeftMouseButtonPressed>,
                         query: Query<(Entity, &Tile)>
) {
    for ev in ev_clicked.read() {
        for (entity, xp) in query.iter() {
            //TODO: Check which tile is clicked
            eprintln!("Clicked at {}x{}", cursor_position.0[0], cursor_position.0[1])
        }
    }
}