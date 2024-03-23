use bevy::{
    prelude::*
};
use rand::{Rng, thread_rng};

use crate::components::{score_components};
use crate::components::gameplay_components;
use crate::components::view_components::{AnimationIndices, AnimationTimer, TileClickedEvent};
use crate::config::{GameplayConfig};


pub fn test_gameplay(mut scoreboard: ResMut<score_components::Scoreboard>,
                     gameplay_config: Res<GameplayConfig>) {
    scoreboard.score += gameplay_config.increment;
}

pub fn spawn_board(mut commands: Commands, gameplay_config: Res<GameplayConfig>) {
    for x in 0..gameplay_config.board_size {
        for y in 0..gameplay_config.board_size {
            let random_tile_type = thread_rng().gen_range(0..gameplay_config.tile_types_count);
            spawn_tile(&mut commands, x, y, random_tile_type);
        }
    }
}

fn spawn_tile(mut commands: &mut Commands, x: usize, y: usize, tile_type: usize) {
    commands.spawn((
        gameplay_components::Tile::new(x, y, tile_type),
        gameplay_components::NeedsView
    ));
}


pub fn mark_selected_tile(world: &World,
                          mut ev_tile_clicked: EventReader<TileClickedEvent>) {
    for ev in ev_tile_clicked.read() {
        let entity_id = world.entity(ev.tile_entity);
        let timer = entity_id.get::<AnimationTimer>().unwrap();
        let texture_atlas = entity_id.get::<TextureAtlas>().unwrap();
        let indices = entity_id.get::<AnimationIndices>().unwrap();
        if timer.0.paused() {
            timer.0.unpause();
        } else {
            timer.0.pause();
            texture_atlas.index = indices.first;
        }
    }
}
