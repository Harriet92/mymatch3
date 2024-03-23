use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::input_components::LeftMouseButtonPressed;
use crate::components::input_components::*;
use crate::components::view_components::MainCamera;

pub fn read_current_cursor_position_system(
    mut coords: ResMut<CurrentWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        coords.value = world_position;
    }
}

pub fn mouse_input_handling_system(buttons: Res<ButtonInput<MouseButton>>,
                                   mut ev_left_click: EventWriter<LeftMouseButtonPressed>) {
    if buttons.just_pressed(MouseButton::Left) {
    }
    if buttons.just_released(MouseButton::Left) {
        ev_left_click.send(LeftMouseButtonPressed);
    }
    if buttons.pressed(MouseButton::Right) {
    // Right Button is being held down
    }
    // we can check multiple at once with `.any_*`
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
    // Either the left or the right button was just pressed
    }
    }