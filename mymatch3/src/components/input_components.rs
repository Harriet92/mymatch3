use bevy::math::Vec2;
use bevy::prelude::{Component, Event, Resource};

#[derive(Resource, Default)]
pub struct CurrentWorldCoords {
    pub value: Vec2
}

impl CurrentWorldCoords{
    pub fn new(x: f32, y:f32) -> CurrentWorldCoords{
        CurrentWorldCoords { value: Vec2::new(x, y) }
    }
}

#[derive(Event)]
pub struct LeftMouseButtonPressed;