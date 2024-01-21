
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::components::*;

pub fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // circular base
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Circle::new(100.0).into()),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
        Collider::cylinder(0.0, 100.0),
        RigidBody::Fixed,
        TransformBundle::from(Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))),
        ));
    });
    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            material: materials.add(Color::rgb_u8(124, 144, 255).into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Fixed,
    ));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });
}