
use bevy::{prelude::*, input::mouse::MouseMotion};

use super::components::*;

const MOVMENT_SPEED: f32 = 5.0;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            projection: PerspectiveProjection {
                fov: 90.0_f32.to_radians(),
                ..default()
            }.into(),
            transform: Transform::from_xyz(0.0, 3.0, 3.0)
                        .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Player {},
    ));
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
    mut player_query: Query<&mut Transform, With<Player>>,
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
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let mut movment_direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            movment_direction += player_transform.forward();
        }
        if keyboard_input.pressed(KeyCode::S) {
            movment_direction += player_transform.back();
        }
        if keyboard_input.pressed(KeyCode::A) {
            movment_direction += player_transform.left();
        }
        if keyboard_input.pressed(KeyCode::D) {
            movment_direction += player_transform.right();
        }

        player_transform.translation += 
            movment_direction * MOVMENT_SPEED * time.delta_seconds();
    }
}

