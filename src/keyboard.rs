use bevy::{prelude::*, input::mouse::MouseMotion};

use crate::gamepad::Inputs;
pub struct KeyboardControllerPlugin;

impl Plugin for KeyboardControllerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(keyboard_mouse_connections);
    }
}

fn keyboard_mouse_connections(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    mut motion_evr: EventReader<MouseMotion>,
    inputs: Res<Inputs>
) {
    let mut horizontal = 0.;
    let mut vertical = 0.;

    if kb.pressed(KeyCode::W) {
        horizontal = 1.;
    }
    if kb.pressed(KeyCode::S) {
        horizontal = -1.;
    }
    if kb.pressed(KeyCode::A) {
        vertical = -1.;
    }
    if kb.pressed(KeyCode::D) {
        vertical = 1.;
    }

    let mut new_inputs = inputs.clone();
    new_inputs.player_movement = Vec2::new(vertical, horizontal);

    if motion_evr.is_empty() {
        new_inputs.camera_movement = Vec2::new(0.,0.);
    } else {
        for ev in motion_evr.iter() {
            new_inputs.camera_movement = Vec2::new(ev.delta.x, ev.delta.y);
        }
    }

    commands.remove_resource::<Inputs>();
    commands.insert_resource(new_inputs);
}

