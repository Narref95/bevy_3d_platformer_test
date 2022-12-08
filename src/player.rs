use bevy::prelude::*;

use crate::{camera::{CameraFollow, MainCamera}, gamepad::Inputs};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

const PLAYER_SPEED: f32 = 7.5;

#[derive(Component)]
pub struct PlayerMovementIndicator;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(player_spawn_system)
        .add_system(player_movement_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule { depth: 0.5, ..default() })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.75, 0.0),
        ..default()
    }).with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.75, 0.0),
            ..default()
        });
    })
    .insert(Player)
    .insert(CameraFollow);

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.25 })),
        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }).insert(PlayerMovementIndicator);
}

fn player_movement_system(
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut target_query: Query<&mut Transform, (With<PlayerMovementIndicator>, Without<Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>, Without<PlayerMovementIndicator>)>,
    inputs: Res<Inputs>
) {
        if let Ok(mut target_transform) = target_query.get_single_mut() {
            if let Ok(mut player_transform) = player_query.get_single_mut() {
                if let Ok(camera_transform) = camera_query.get_single_mut() {
                    let move_right = (inputs.player_movement.x * time.delta_seconds() * PLAYER_SPEED) * camera_transform.right();
                    let move_forward = (inputs.player_movement.y * time.delta_seconds() * PLAYER_SPEED) * camera_transform.forward();
                    let player_final_pos = player_transform.translation + move_right + move_forward;
                    let mut target_final_pos = player_transform.translation + (move_right + move_forward) * 17.5;
                    target_final_pos.y = 0.;
                    player_transform.translation.x = player_final_pos.x;
                    player_transform.translation.z = player_final_pos.z;

                    target_transform.translation = target_final_pos;
                    if inputs.player_movement.x != 0. || inputs.player_movement.y != 0. {
                        let player_y = player_transform.translation.y;
                        player_transform.look_at(Vec3 {x: target_final_pos.x, y: player_y, z: target_final_pos.z}, Vec3::Y);                    
                    }
                }
            }
        }
}