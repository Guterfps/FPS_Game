
use bevy::{
    prelude::{App, Plugin, IntoSystemSetConfigs, SystemSet}, 
    app::{Startup, Update}, ecs::schedule::IntoSystemConfigs,
};

mod components;
mod systems;

use systems::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movment,
    Confinement,
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .configure_sets(Update, 
            PlayerSystemSet::Movment
                    .before(PlayerSystemSet::Confinement))
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (
            (update_player_rotation,
            update_player_movment).in_set(PlayerSystemSet::Movment),
            (update_player_grounded, 
            update_player_speed).chain().in_set(PlayerSystemSet::Confinement),
            )
        );
    }
}