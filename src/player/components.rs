
use bevy::prelude::{Component, Vec3};

pub const MOVMENT_SPEED_SLOW_DOWN: f32 = 25.0;
pub const MAX_MOVMENT_SPEED: f32 = 10.0;
pub const JUMP_FORCE: f32 = 5.0;
pub const GRAVITY: f32 = -9.81;

#[derive(Component, Default)]
pub struct Player {
    pub speed: Vec3,
    pub is_grounded: bool,
}

#[derive(Component)]
pub struct PlayerCamera;