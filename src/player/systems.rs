
use bevy::{prelude::*, input::mouse::MouseMotion};
use bevy_rapier3d::prelude::*;

use super::components::*;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, 3.0, 4.0)),
        // RigidBody::KinematicPositionBased,
        Collider::capsule_y(1.0, 0.5),
        KinematicCharacterController::default(),
        Player {..default()},
    )).with_children(|parent| {
        parent.spawn((
            Camera3dBundle {
                projection: PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..default()
                }.into(),
                transform: Transform::from_xyz(0.0, 1.5, 0.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            PlayerCamera {}
        ));
    });
}

pub fn despawn_player_camera(
    mut commands: Commands, 
    camera_query: Query<Entity, With<Player>>
) {
    if let Ok(camera_entity) = camera_query.get_single() {
        commands.entity(camera_entity).despawn_recursive();
    }
}

pub fn update_player_rotation(
    mut player_query: Query<&mut Transform, With<PlayerCamera>>,
    mut mouse_events: EventReader<MouseMotion>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        mouse_events.read().for_each(|event| {
            player_transform.rotate_local_x(-event.delta.y * 0.001);
            player_transform.rotate_y(-event.delta.x * 0.001);
        });
    }
}

pub fn update_player_position(
    mut player_query: Query<
                (&mut KinematicCharacterController, &mut Player), With<Player>>,
    player_camera_query: Query<&Transform, With<PlayerCamera>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    if let Ok(camera_transform) = player_camera_query.get_single() {
        for (mut controller, mut player) in player_query.iter_mut() {
            if player.is_grounded {
                if keyboard_input.pressed(KeyCode::W) {
                    player.speed += Vec3::new(camera_transform.forward().x, 
                    0.0, camera_transform.forward().z)
                    .normalize();
                }
                if keyboard_input.pressed(KeyCode::S) {
                    player.speed += Vec3::new(camera_transform.back().x, 
                 0.0, camera_transform.back().z)
                    .normalize();
                }
                if keyboard_input.pressed(KeyCode::A) {
                    player.speed += Vec3::new(camera_transform.left().x, 
                    0.0, camera_transform.left().z)
                    .normalize();   
                }
                if keyboard_input.pressed(KeyCode::D) {
                    player.speed += Vec3::new(camera_transform.right().x, 
                    0.0, camera_transform.right().z)
                    .normalize();
                }
                if keyboard_input.pressed(KeyCode::Space) {
                    player.speed.y += JUMP_FORCE;
                }
            }
            
            let time_delta = time.delta_seconds();
            let mut movment_translation = player.speed * time_delta;
            movment_translation.y = calc_player_y_movment(&player, time_delta);
            player.speed.y = calc_player_y_speed(&player, time_delta);
            
            controller.translation = Some(movment_translation);
        }
    }
}

pub fn update_player_grounded(
    mut player_query: Query<
                    (&KinematicCharacterControllerOutput, &mut Player), 
                    With<Player>>,
) {
    for (output, mut player) in player_query.iter_mut() {
        player.is_grounded = output.grounded;
    }
}

pub fn update_player_speed(
 mut player_query: Query<&mut Player, With<Player>>,
 time: Res<Time>
) {
    for mut player in player_query.iter_mut() {
        player.speed = player.speed.clamp_length_max(MAX_MOVMENT_SPEED);
        
        if player.is_grounded {
            let direction = player.speed.xz().normalize_or_zero();
            let delta_time = time.delta_seconds();
            player.speed.x -= direction.x * MOVMENT_SPEED_SLOW_DOWN * delta_time;
            player.speed.z -= direction.y * MOVMENT_SPEED_SLOW_DOWN * delta_time;
        }
    }
}

fn calc_player_y_movment(player: &Player, time_delta: f32) -> f32 {
    (player.speed.y * time_delta) + (GRAVITY * time_delta * time_delta / 2.0)
}

fn calc_player_y_speed(player: &Player, time_delta: f32) -> f32 {
    player.speed.y + (GRAVITY * time_delta)
}