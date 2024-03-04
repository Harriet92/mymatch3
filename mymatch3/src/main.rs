use bevy::{
    prelude::*
};

mod systems;
mod components;

use crate::systems::*;
use crate::components::score_components::Scoreboard;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .add_systems(Startup, setup::spawn_ui_system)
        /*.add_systems(
            FixedUpdate,
            ()
                // `chain`ing systems together runs them in order
                .chain(),
        )*/
        .add_systems(Update, gameplay::test_gameplay)
        .add_systems(Update, gui_systems::update_scoreboard)
        .run();
}
