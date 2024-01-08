
use bevy::{prelude::*, input::mouse::MouseMotion};

use super::components::PlayerCamera;

const MOVMENT_SPEED: f32 = 5.0;

pub fn spawn_player_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            projection: PerspectiveProjection {
                fov: 90.0_f32.to_radians(),
                ..default()
            }.into(),
            transform: Transform::from_xyz(-2.0, 4.0, 9.0)
                        .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PlayerCamera {},
    ));
}

pub fn despawn_player_camera(
    mut commands: Commands, 
    camera_query: Query<Entity, With<PlayerCamera>>
) {
    if let Ok(camera_entity) = camera_query.get_single() {
        commands.entity(camera_entity).despawn_recursive();
    }
}

pub fn update_player_camera_rotation(
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
    mut mouse_events: EventReader<MouseMotion>,
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        mouse_events.read().for_each(|event| {
            camera_transform.rotate_x(-event.delta.y * 0.001);
            camera_transform.rotate_y(event.delta.x * 0.001);
        });
    }
}

pub fn update_player_camera_position(
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        let mut movment_direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            movment_direction += camera_transform.forward();
        }
        if keyboard_input.pressed(KeyCode::S) {
            movment_direction += camera_transform.back();
        }
        if keyboard_input.pressed(KeyCode::A) {
            movment_direction += camera_transform.left();
        }
        if keyboard_input.pressed(KeyCode::D) {
            movment_direction += camera_transform.right();
        }

        camera_transform.translation += 
            movment_direction * MOVMENT_SPEED * time.delta_seconds();
    }
}

