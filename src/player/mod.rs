
use bevy::{prelude::{App, Plugin}, app::{Startup, Update}};

mod components;
mod systems;

use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (
            update_player_rotation,
            update_player_position,
            update_player_grounded,
            )
        );
    }
}