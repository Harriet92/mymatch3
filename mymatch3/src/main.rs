use bevy::{
    prelude::*
};

mod systems;
use crate::systems::setup_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_system::setup)
        /*.add_systems(
            FixedUpdate,
            ()
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(Update, ())*/
        .run();

}
