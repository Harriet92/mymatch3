use bevy::{
    prelude::*
};
use bevy::window::WindowRef::Entity;
use rand::{Rng, thread_rng};

use crate::components::{score_components};
use crate::components::gameplay_components;
use crate::components::gameplay_components::{Selected, Tile};
use crate::components::view_components::{AnimationIndices, AnimationTimer, MainCamera, TileClickedEvent};
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


pub fn mark_selected_tile(mut ev_tile_clicked: EventReader<TileClickedEvent>,
                          mut commands: &mut Commands,
                          mut q_tiles: Query<(Entity, &gameplay_components::Tile)>,
                          q_selected_tiles: Query<(Entity, &gameplay_components::Tile), With<Selected>> ) {
    for ev in ev_tile_clicked.read() {
        let mut clicked_tile_entity: Option<Entity> = None;
        for (tile_entity, tile) in q_tiles.iter_mut() {
            if tile.x == ev.x && tile.y == ev.y {
                clicked_tile_entity = tile_entity;
                break;
            }
        }
        if clicked_tile_entity == None {
            return;
        }
        let selected_count = q_selected_tiles.iter().count();
        if selected_count == 0 {
            set_tile_as_selected(commands, clicked_tile_entity.unwrap());
            return;
        } else if selected_count == 2 {
            set_tile_as_selected(commands, clicked_tile_entity.unwrap());
            for (tile_entity, _) in q_selected_tiles.iter() {
                commands.entity(tile_entity).remove::<Selected>();
            }
        } else {
            set_tile_as_selected(commands, clicked_tile_entity.unwrap());
            let (_, selected_tile) = q_selected_tiles.single();
            if
        }


        /*if timer.0.paused() {
            timer.0.unpause();
        } else {
            timer.0.pause();
            texture_atlas.index = indices.first;
        }*/
    }
}

fn set_tile_as_selected(mut commands: &mut Commands, tile_entity: Entity) {
    commands.entity(tile_entity).insert(Selected);
}

fn are_adjacent(t1: Tile, t2: Tile) -> bool {
    
}