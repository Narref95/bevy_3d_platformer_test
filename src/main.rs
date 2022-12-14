use bevy::{prelude::*, render::settings::{WgpuSettings, WgpuFeatures}, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};
use bevy_rapier3d::prelude::*;
use bevy_editor_pls::EditorPlugin;
use camera::CameraPlugin;
use gamepad::{GamepadControllerPlugin, Inputs};
use keyboard::KeyboardControllerPlugin;
use player::PlayerPlugin;

mod camera;
mod gamepad;
mod keyboard;
mod player;

pub const HEIGHT: f32 = 1080.0;
pub const RATIO: f32 = 16. / 9.;

fn main() {
    let wpu_settings: WgpuSettings = WgpuSettings {
        features: WgpuFeatures::POLYGON_MODE_LINE,
        ..default()
    };
    let window_plugin: WindowPlugin = WindowPlugin {
        window: WindowDescriptor {
            width: HEIGHT * RATIO,
            height: HEIGHT,
            title: "Fall Guys xd".to_string(),
            position: WindowPosition::Centered,
            ..default()
        },
        ..default()
    };
    App::new()
    .insert_resource(wpu_settings)
    .insert_resource(AmbientLight {
        color: Color::rgb(0.5, 0.5, 0.5),
        brightness: 2.5
    })
    .insert_resource(Inputs::default())
    .add_plugins(DefaultPlugins.set(window_plugin))
    .add_plugin(EditorPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(CameraPlugin)
    // .add_plugin(GamepadControllerPlugin)
    .add_plugin(KeyboardControllerPlugin)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_startup_system(setup)
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 25.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    }).insert(Collider::cuboid(12.5, 0.1, 12.5));

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 25.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(20.0, 1.0, 0.0).with_rotation(Quat::from_rotation_z(0.15)),
        ..default()
    }).insert(Collider::cuboid(12.5, 0.1, 12.5));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(10.0, 0.75, 0.0),
        ..default()
    });
}