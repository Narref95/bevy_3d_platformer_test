use bevy::prelude::*;

/// Simple resource to store the ID of the connected gamepad.
/// We need to know which gamepad to use for player input.
#[derive(Resource)]
struct MyGamepad(Gamepad);

#[derive(Clone, Resource)]
pub struct Inputs {
    // Player movement
    pub player_movement: Vec2,
    // Camera movement
    pub camera_movement: Vec2,
    pub camera_angle: f32,
    pub jump_button: bool,
    pub dash_button: bool,
}

impl Inputs {
    pub fn default() -> Self {
        Self {
            camera_movement: Vec2::default(),
            player_movement: Vec2::default(),
            camera_angle: 0.,
            jump_button: false,
            dash_button: false
        }
    }
}
pub struct GamepadControllerPlugin;

impl Plugin for GamepadControllerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(gamepad_connections)
        .add_system(gamepad_movement);
    }
}

fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for ev in gamepad_evr.iter() {
        // the ID of the gamepad
        let id = ev.gamepad;
        match &ev.event_type {
            GamepadEventType::Connected(info) => {
                println!("New gamepad connected with ID: {:?}, name: {}", id, info.name);

                // if we don't have any gamepad yet, use this one
                dbg!(my_gamepad.is_none());
                if my_gamepad.is_none() {
                    println!("Adding gamepad resource");
                    commands.insert_resource(MyGamepad(id));
                }
            }
            GamepadEventType::Disconnected => {
                println!("Lost gamepad connection with ID: {:?}", id);

                // if it's the one we previously associated with the player,
                // disassociate it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if *old_id == id {
                        println!("Removing gamepad resource");
                        commands.remove_resource::<MyGamepad>();
                    }
                }
            }
            // other events are irrelevant
            _ => {}
        }
    }
}

fn gamepad_movement(
    mut commands: Commands,
    axes: Res<Axis<GamepadAxis>>,
    my_gamepad: Option<Res<MyGamepad>>,
    inputs: Res<Inputs>
) {
    let gamepad = if let Some(gp) = my_gamepad {
        gp.0
    } else {
        return;
    };

    // The joysticks are represented using a separate axis for X and Y
    let axis_lx = GamepadAxis {
        gamepad, axis_type: GamepadAxisType::LeftStickX
    };
    let axis_ly = GamepadAxis {
        gamepad, axis_type: GamepadAxisType::LeftStickY
    };

    let axis_rx = GamepadAxis {
        gamepad, axis_type: GamepadAxisType::RightStickX
    };
    let axis_ry = GamepadAxis {
        gamepad, axis_type: GamepadAxisType::RightStickY
    };

    let mut new_inputs = inputs.clone();

    if let (Some(x), Some(z)) = (axes.get(axis_lx), axes.get(axis_ly)) {
        new_inputs.player_movement = Vec2::new(x, z);
    }

    if let (Some(x), Some(z)) = (axes.get(axis_rx), axes.get(axis_ry)) {
        new_inputs.camera_movement = Vec2::new(x, z);
    }

    commands.remove_resource::<Inputs>();
    commands.insert_resource(new_inputs);
}