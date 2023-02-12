#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::{prelude::*, render::settings::{WgpuSettings, WgpuFeatures}, diagnostic::{FrameTimeDiagnosticsPlugin}, window::PresentMode};
use bevy_rapier3d::prelude::*;
use bevy_editor_pls::EditorPlugin;
use camera::CameraPlugin;
use debug_mode::DebugModePlugin;
use gamepad::{GamepadControllerPlugin, Inputs};
use keyboard::KeyboardControllerPlugin;
use player::PlayerPlugin;

mod camera;
mod gamepad;
mod keyboard;
mod player;
mod character_controller;
mod debug_mode;

pub const HEIGHT: f32 = 720.0;
pub const RATIO: f32 = 16. / 9.;

const GROUND_COLLISION: CollisionGroups = CollisionGroups::new(Group::GROUP_1, Group::GROUP_10);

fn main() {
    let wpu_settings: WgpuSettings = WgpuSettings {
        features: WgpuFeatures::POLYGON_MODE_LINE,
        ..default()
    };
    let window_plugin: WindowPlugin = WindowPlugin {
        window: WindowDescriptor {
            width: HEIGHT * RATIO,
            height: HEIGHT,
            title: "Stinky Guys".to_string(),
            position: WindowPosition::Centered,
            present_mode: PresentMode::AutoVsync,
            ..default()
        },
        ..default()
    };
    App::new()
    .register_type::<Group>()
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
    //.add_plugin(GamepadControllerPlugin)
    .add_plugin(KeyboardControllerPlugin)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(RapierDebugRenderPlugin::default())
    // .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(DebugModePlugin)
    .add_startup_system(setup)
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 25.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }).insert(Collider::cuboid(12.5, 0.1, 12.5))
    .insert(GROUND_COLLISION);

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 25.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(20.0, 0.0, 0.0).with_rotation(Quat::from_rotation_z(0.15)),
        ..default()
    }).insert(Collider::cuboid(12.5, 0.1, 12.5))
    .insert(GROUND_COLLISION);

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 25. })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(50.0, -10.5, 15.0),
        ..default()
    }).insert(Collider::cuboid(12.5, 12.5, 12.5))
    .insert(GROUND_COLLISION);

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 25. })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(50.0, -10.5, -35.0),
        ..default()
    }).insert(Collider::cuboid(12.5, 12.5, 12.5))
    .insert(GROUND_COLLISION);

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

    commands.spawn(DynamicSceneBundle{
        scene: asset_server.load("scenes/scene.scn.ron"),
        ..default()
    });
}