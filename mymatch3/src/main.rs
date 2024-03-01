use bevy::{
    prelude::*
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        /*.add_systems(Startup, ())
        .add_systems(
            FixedUpdate,
            ()
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(Update, ())*/
        .run();

}
