
use bevy::{prelude::{App, Plugin}, app::{Startup, Update}};

mod components;
mod systems;

use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_player_camera)
        .add_systems(Update, (
            update_player_camera_rotation,
            update_player_camera_position
            )
        )
        ;
    }
}