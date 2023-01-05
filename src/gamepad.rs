use bevy::{prelude::*, input::gamepad::{GamepadSettings, AxisSettings, ButtonSettings}};

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
        .add_startup_system(configure_gamepads)
        .add_system(gamepad_connections)
        .add_system(gamepad_movement);
    }
}

// this should be run once, when the game is starting
// (transition entering your in-game state might be a good place to put it)
fn configure_gamepads(
    my_gamepad: Option<Res<MyGamepad>>,
    mut settings: ResMut<GamepadSettings>,
) {
    let gamepad = if let Some(gp) = my_gamepad {
        // a gamepad is connected, we have the id
        gp.0
    } else {
        // no gamepad is connected
        return;
    };

    // add a larger default dead-zone to all axes (ignore small inputs, round to zero)
    settings.default_axis_settings.set_deadzone_lowerbound(-0.5);
    settings.default_axis_settings.set_deadzone_upperbound(0.5);
    settings.default_axis_settings.set_livezone_lowerbound(-0.2);
    settings.default_axis_settings.set_livezone_upperbound(0.2);

    // make the right stick "binary", squash higher values to 1.0 and lower values to 0.0
    // let mut right_stick_settings = AxisSettings::default();
    // right_stick_settings.set_deadzone_lowerbound(-0.5);
    // right_stick_settings.set_deadzone_upperbound(0.5);
    // right_stick_settings.set_livezone_lowerbound(-0.5);
    // right_stick_settings.set_livezone_upperbound(0.5);
    // the raw value should change by at least this much,
    // for Bevy to register an input event:
    // right_stick_settings.set_threshold(0.01);

    // make the triggers work in big/coarse steps, to get fewer events
    // reduces noise and precision
    let mut trigger_settings = AxisSettings::default();
    trigger_settings.set_threshold(0.25);

    // set these settings for the gamepad we use for our player
    // settings.axis_settings.insert(
    //     GamepadAxis { gamepad, axis_type: GamepadAxisType::RightStickX },
    //     settings.default_axis_settings.clone()
    // );
    // settings.axis_settings.insert(
    //     GamepadAxis { gamepad, axis_type: GamepadAxisType::RightStickY },
    //     settings.default_axis_settings.clone()
    // );
    // settings.axis_settings.insert(
    //     GamepadAxis { gamepad, axis_type: GamepadAxisType::LeftStickX },
    //     settings.default_axis_settings.clone()
    // );
    // settings.axis_settings.insert(
    //     GamepadAxis { gamepad, axis_type: GamepadAxisType::LeftStickY },
    //     settings.default_axis_settings.clone()
    // );
    // settings.axis_settings.insert(
    //     GamepadAxis { gamepad, axis_type: GamepadAxisType::LeftZ },
    //     trigger_settings.clone()
    // );
    // settings.axis_settings.insert(
    //     GamepadAxis { gamepad, axis_type: GamepadAxisType::RightZ },
    //     trigger_settings.clone()
    // );

    // for buttons (or axes treated as buttons):
    let mut button_settings = ButtonSettings::default();
    // require them to be pressed almost all the way, to count
    button_settings.set_press_threshold(0.9);
    // require them to be released almost all the way, to count
    button_settings.set_release_threshold(0.1);

    settings.default_button_settings = button_settings;
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
    buttons: Res<Input<GamepadButton>>,
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

    // In a real game, the buttons would be configurable, but here we hardcode them
    let jump_button = GamepadButton {
        gamepad, button_type: GamepadButtonType::South
    };
    let dash_button = GamepadButton {
        gamepad, button_type: GamepadButtonType::West
    };

    let mut new_inputs = inputs.clone();
    new_inputs.jump_button = buttons.just_pressed(jump_button);
    new_inputs.dash_button = buttons.just_pressed(dash_button);

    if let (Some(x), Some(z)) = (axes.get(axis_lx), axes.get(axis_ly)) {
        new_inputs.player_movement = Vec2::new(x, z);
    }

    if let (Some(x), Some(z)) = (axes.get(axis_rx), axes.get(axis_ry)) {
        new_inputs.camera_movement = Vec2::new(x, z);
    }

    commands.remove_resource::<Inputs>();
    commands.insert_resource(new_inputs);
}