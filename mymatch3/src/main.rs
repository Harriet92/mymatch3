use bevy::{
    prelude::*
};

mod systems;
use crate::systems::setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup::spawn_ui_system)
        /*.add_systems(
            FixedUpdate,
            ()
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(Update, ())*/
        .run();

}
