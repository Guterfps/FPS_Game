
use bevy::prelude::Component;

pub const MOVMENT_SPEED: f32 = 5.0;
pub const JUMP_FORCE: f32 = 5.0;
pub const GRAVITY: f32 = -9.81;

#[derive(Component, Default)]
pub struct Player {
    pub speed_y: f32,
    pub is_grounded: bool,
}

#[derive(Component)]
pub struct PlayerCamera;