use bevy::math::Vec2;
use bevy::prelude::{Component, Resource};

#[derive(Resource, Default)]
pub struct CurrentWorldCoords(pub Vec2);
