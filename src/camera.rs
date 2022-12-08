use bevy::prelude::*;

use crate::{gamepad::Inputs};

pub struct CameraPlugin;

#[derive(Component)]
pub struct MainCamera {
    angle: f32
}

#[derive(Component)]
pub struct CameraFollow;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_camera)
        .add_system(camera_movement);
    }
}

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-15.0, 5.0, 0.0).looking_at(Vec3 { x: 0., y: 1.5, z: 0.}, Vec3::Y),
        ..default()
    }).insert(MainCamera {
        angle: 0.
    });
}

fn camera_movement(
    time: Res<Time>,
    mut query_camera: Query<(&mut Transform, &mut MainCamera), With<MainCamera>>,
    query_target: Query<&mut Transform, (With<CameraFollow>, Without<MainCamera>)>,
    inputs: Res<Inputs>
) {
    if let Ok(target_transform) = query_target.get_single() {
        for (mut camera_transform, mut camera) in query_camera.iter_mut() {
            camera.angle += inputs.camera_movement.x * time.delta_seconds() * 2.;
            camera_transform.translation = target_transform.translation + Vec3::new(-15.,5.,0.);
            camera_transform.rotate_around(target_transform.translation, Quat::from_rotation_y(camera.angle));
            camera_transform.look_at(target_transform.translation, Vec3::Y);
        }
    }
}