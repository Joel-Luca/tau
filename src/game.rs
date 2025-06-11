use bevy::prelude::*;

use crate::ability;
use crate::configuration;
use crate::enemy;
use crate::physic;
use crate::player;
use crate::projectile;
use crate::tank;
use crate::wall;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ability::AbilityPlugin,
            configuration::ConfigurationPlugin,
            enemy::EnemyPlugin,
            physic::PhysicPlugin,
            player::PlayerPlugin,
            projectile::ProjectilePlugin,
            tank::TankPlugin,
            wall::WallPlugin,
        ))
        .add_systems(Startup, setup_scene);
    }
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d { ..default() });
}
