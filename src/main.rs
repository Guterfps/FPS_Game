
use bevy::prelude::{App, DefaultPlugins};
use bevy_rapier3d::prelude::{
    RapierPhysicsPlugin, 
    RapierDebugRenderPlugin, 
    NoUserData
};

mod player;
mod world;

use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins,
        RapierPhysicsPlugin::<NoUserData>::default(),
        RapierDebugRenderPlugin::default(),
        PlayerPlugin,
        WorldPlugin))
    .run();
}
