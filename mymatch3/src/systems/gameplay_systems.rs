use bevy::{
    prelude::*
};
use bevy::ecs::entity::Entity;
use rand::{Rng, thread_rng};

use crate::components::{score_components};
use crate::components::gameplay_components::*;
use crate::components::view_components::{TileClickedEvent};
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
        Tile::new(x, y, tile_type),
        NeedsView
    ));
}


pub fn mark_selected_tile(mut ev_tile_clicked: EventReader<TileClickedEvent>,
                          mut commands: Commands,
                          q_tiles: Query<(Entity, &Tile)>,
                          q_selected_tiles: Query<(Entity, &Tile), With<Selected>> ) {
    for ev in ev_tile_clicked.read() {
        let mut clicked_tile_entity: Option<Entity> = None;
        let mut clicked_tile_component: Option<&Tile> = None;
        for (tile_entity, tile) in q_tiles.iter() {
            if tile.x == ev.x && tile.y == ev.y {
                clicked_tile_entity = Option::from(tile_entity);
                clicked_tile_component = Option::from(tile);
                break;
            }
        }
        if clicked_tile_entity == None {
            return;
        }
        let selected_count = q_selected_tiles.iter().count();
        set_tile_as_selected(&mut commands, clicked_tile_entity.unwrap());

        match selected_count {
            0 => return,
            1 => {
                let (entity, selected_tile) = q_selected_tiles.single();
                if are_adjacent(selected_tile, clicked_tile_component.unwrap()) == false{
                    commands.entity(entity).remove::<Selected>();
                }
            },
            2 => {
                for (tile_entity, _) in q_selected_tiles.iter() {
                    commands.entity(tile_entity).remove::<Selected>();
                }
            },
            _ => panic!("Selected tiles number exceeds 2, something went wrong!")
        }
    }
}

fn set_tile_as_selected(mut commands: &mut Commands, tile_entity: Entity) {
    commands.entity(tile_entity).insert(Selected);
}

fn are_adjacent(t1: &Tile, t2: &Tile) -> bool {
    let dx: i32 = (t1.x as i32 - t2.x as i32).abs();
    let dy: i32 = (t1.y as i32 - t2.y as i32).abs();
    return dx + dy == 1;
}

pub fn switch_tile_places(mut commands: Commands,
                          q_selected_tiles: Query<(Entity, &mut Tile), With<Selected>> ) {

    let selected_count = q_selected_tiles.iter().count();
    if selected_count != 2 {
        return;
    }
    // switch tiles' indexes
    // create a view system which will lerp the position of the sprites

}
