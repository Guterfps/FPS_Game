
use bevy::{prelude::{App, Plugin}, app::Startup};

mod components;
mod systems;

use systems::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_world);
    }
}