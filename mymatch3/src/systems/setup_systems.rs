use bevy::{
    prelude::*,
    ecs::system::Commands
};
use crate::components::view_components::*;

pub fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}