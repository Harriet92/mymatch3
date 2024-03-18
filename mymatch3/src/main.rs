use bevy::{
    prelude::*
};
use bevy_rapier2d::prelude::*;
use components::input_components::CurrentWorldCoords;
use crate::components::gameplay_components::LeftMouseButtonPressed;

mod systems;
mod components;
mod config;

use crate::systems::*;
use crate::components::score_components::Scoreboard;
use crate::config::{GameplayConfig, GameplayViewConfig};

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(Scoreboard { score: 0 })
        .init_resource::<CurrentWorldCoords>()
        .insert_resource(GameplayConfig::default())
        .insert_resource(GameplayViewConfig::default(GameplayConfig::default().board_size))
        .add_event::<LeftMouseButtonPressed>()
        .add_systems(Startup, (gui_systems::spawn_ui_system, setup_systems::spawn_camera))
        .add_systems(PostStartup, gameplay_systems::spawn_board)
        /*.add_systems(
            FixedUpdate,
            ()
                // `chain`ing systems together runs them in order
                .chain(),
        )*/
        .add_systems(Update, (input_systems::read_current_cursor_position_system, input_systems::mouse_input_handling_system))
        .add_systems(Update, (gameplay_view_systems::spawn_tile_images, gameplay_systems::mark_clicked_tile))
        .add_systems(Update, (view_systems::animate_sprite))
        .add_systems(Update, gui_systems::update_scoreboard)
        .run();
}
