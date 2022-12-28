use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::prelude::Group};

use crate::{camera::{CameraFollow, MainCamera}, gamepad::Inputs};

pub struct PlayerPlugin;

#[derive(Component, Clone)]
pub struct Player {
    pub is_jumping: bool,
    pub is_grounded: bool,
    pub dashes: i8
}

const PLAYER_SPEED: f32 = 10.;
const JUMP_HEIGHT: f32 = 9.;

#[derive(Component)]
pub struct PlayerMovementIndicator;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(player_spawn_system)
        .add_system(player_movement_system)
        .add_system(player_jump_system)
        .add_system(check_is_grounded);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>
) {

    let my_gltf = ass.load("models\\stickman.glb#Scene0");

    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    commands.spawn(SceneBundle {
        scene: my_gltf,
        transform: Transform::from_xyz(0., 1., 0.).with_scale(Vec3::new(2.,2.,2.)).with_rotation(Quat::from_rotation_y(45.)),
        ..Default::default()
    }).insert(Player {
        is_jumping: false,
        is_grounded: false,
        dashes: 0
    })
    .insert(CameraFollow)
    .insert(RigidBody::Dynamic)
    .insert(Velocity {
        linvel: Vec3::ZERO,
        angvel: Vec3::ZERO,
    })
    .insert(ExternalImpulse {
        impulse: Vec3::ZERO,
        torque_impulse: Vec3::ZERO,
    })
    .with_children(|children| {
        children.spawn(PbrBundle::default())
            .insert(Collider::ball(0.5))
            // Position the collider relative to the rigid-body.
            .insert(TransformBundle::from(Transform::from_xyz(0.0, 1.1, 0.0)))
            .insert(CollisionGroups::new(bevy_rapier3d::geometry::Group::GROUP_10, bevy_rapier3d::geometry::Group::GROUP_1));
    })
    .insert(GravityScale(2.))
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(CollisionGroups::new(bevy_rapier3d::geometry::Group::GROUP_10, bevy_rapier3d::geometry::Group::GROUP_1));

    // player
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Capsule { depth: 0.5, ..default() })),
    //     material: materials.add(Color::WHITE.into()),
    //     transform: Transform::from_xyz(0.0, 0.75, 0.0),
    //     ..default()
    // }).with_children(|parent| {
    //     parent.spawn(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Capsule { radius: 0.2, depth: 0.1, ..default() })),
    //         material: materials.add(Color::BEIGE.into()),
    //         transform: Transform::from_xyz(0., 1., 0.),
    //         ..default()
    //     });
    //     parent.spawn(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Capsule { radius: 0.2, depth: 0.1, ..default() })),
    //         material: materials.add(Color::BEIGE.into()),
    //         transform: Transform::from_xyz(0.5, 0.5, 0.),
    //         ..default()
    //     });
    //     parent.spawn(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Capsule { radius: 0.2, depth: 0.1, ..default() })),
    //         material: materials.add(Color::BEIGE.into()),
    //         transform: Transform::from_xyz(-0.5, 0.5, 0.),
    //         ..default()
    //     });
    // })
    // .insert(Player {
    //     is_jumping: false,
    //     is_grounded: false,
    //     dashes: 0
    // })
    // .insert(CameraFollow)
    // .insert(RigidBody::Dynamic)
    // .insert(Velocity {
    //     linvel: Vec3::ZERO,
    //     angvel: Vec3::ZERO,
    // })
    // .insert(ExternalImpulse {
    //     impulse: Vec3::ZERO,
    //     torque_impulse: Vec3::ZERO,
    // })
    // .insert(GravityScale(1.))
    // .insert(Collider::ball(0.5))
    // .insert(LockedAxes::ROTATION_LOCKED)
    // .insert(CollisionGroups::new(bevy_rapier3d::geometry::Group::GROUP_10, bevy_rapier3d::geometry::Group::GROUP_1));

    // enemy
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule { depth: 0.5, ..default() })),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_xyz(10.0, 0.75, 0.0),
        ..default()
    }).with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule { radius: 0.2, depth: 0.1, ..default() })),
            material: materials.add(Color::BEIGE.into()),
            transform: Transform::from_xyz(0., 1., 0.),
            ..default()
        });
        parent.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule { radius: 0.2, depth: 0.1, ..default() })),
            material: materials.add(Color::BEIGE.into()),
            transform: Transform::from_xyz(0.5, 0.5, 0.),
            ..default()
        });
        parent.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule { radius: 0.2, depth: 0.1, ..default() })),
            material: materials.add(Color::BEIGE.into()),
            transform: Transform::from_xyz(-0.5, 0.5, 0.),
            ..default()
        });
    })
    .insert(RigidBody::Dynamic)
    .insert(Collider::cylinder(0.5, 0.5));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.25 })),
        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }).insert(PlayerMovementIndicator);
}

fn player_jump_system(
    mut player_query: Query<(&mut Velocity, &mut Player), With<Player>>,
    inputs: Res<Inputs>
) {
    if inputs.jump_button{
        for (mut velocity, mut player) in player_query.iter_mut() {
            if !player.is_jumping {
                velocity.linvel = Vec3::new(velocity.linvel.x,JUMP_HEIGHT, velocity.linvel.z);
                player.is_jumping = true;
            }
        }
    }
}

fn player_movement_system(
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    mut target_query: Query<&mut Transform, (With<PlayerMovementIndicator>, Without<Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>, Without<PlayerMovementIndicator>)>,
    inputs: Res<Inputs>
) {
        if let Ok(mut target_transform) = target_query.get_single_mut() {
            for (mut player_transform, mut player_vel) in player_query.iter_mut() {
                if let Ok(camera_transform) = camera_query.get_single_mut() {
                    let move_right = inputs.player_movement.x * PLAYER_SPEED * camera_transform.right();
                    let move_forward = inputs.player_movement.y * PLAYER_SPEED * camera_transform.forward();
                    let mut target_final_pos = player_transform.translation + (move_right / 5. + move_forward / 5.);
                    let mut look_final_pos = player_transform.translation + (-move_right / 5. + -move_forward / 5.);
                    look_final_pos.y = player_transform.translation.y;
                    target_final_pos.y = player_transform.translation.y;
                    player_vel.linvel = (move_right + move_forward) * Vec3::new(1.,0.,1.) + Vec3::new(0., player_vel.linvel.y,0.);
                    
                    target_transform.translation = target_final_pos;
                    if inputs.player_movement.x != 0. || inputs.player_movement.y != 0. {
                        player_transform.look_at(look_final_pos, Vec3::Y);                    
                    }
                }
            }
        }
}

fn check_is_grounded(
    rapier_context: Res<RapierContext>,
    mut player_query: Query<(&mut Player, &Transform), With<Player>>
) {
    for (mut player, player_transform) in player_query.iter_mut() {
        let ray_pos = player_transform.translation;
        let ray_dir = Vec3::new(0., -1., 0.);
        let max_toi = 0.1;
        let solid = true;
        let filter = QueryFilter::default().groups(InteractionGroups::new(Group::GROUP_10, Group::GROUP_1));
    
        rapier_context.intersections_with_ray(
        ray_pos, ray_dir, max_toi, solid, filter,
        |_entity, _intersection| {
            player.is_jumping = false;
            true // Return `false` instead if we want to stop searching for other hits.
        });
    }
}