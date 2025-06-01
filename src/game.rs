use bevy::prelude::*;

use crate::configuration;
use crate::enemy;
use crate::physic;
use crate::player;
use crate::tank;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            configuration::ConfigurationPlugin,
            enemy::EnemyPlugin,
            player::PlayerPlugin,
            physic::PhysicPlugin,
            tank::TankPlugin,
        ))
        .add_systems(Startup, setup_scene);
    }
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d { ..default() });
}
