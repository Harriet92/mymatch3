use bevy::{
    prelude::*
};

mod systems;
mod components;
mod config;

use crate::systems::*;
use crate::components::score_components::Scoreboard;
use crate::config::GameplayConfig;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(GameplayConfig::default())
        .add_systems(Startup, (gui_systems::spawn_ui_system, setup::load_assets))
        /*.add_systems(
            FixedUpdate,
            ()
                // `chain`ing systems together runs them in order
                .chain(),
        )*/
        .add_systems(Update, (gameplay::test_gameplay, gui_systems::animate_sprite))
        .add_systems(Update, gui_systems::update_scoreboard)
        .run();
}
