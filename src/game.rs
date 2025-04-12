use bevy::prelude::*;

use crate::enemy;
use crate::player;
use crate::projectile;
use crate::resolution;

pub struct GamePlugin; 

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
                (
                    enemy::EnemyPlugin,
                    player::PlayerPlugin,
                    projectile::ProjectilePlugin,
                    resolution::ResolutionPlugin
                )
            )
            .add_systems(Startup, setup_scene);
    }
}

fn setup_scene(mut commands : Commands) {
    commands.spawn(Camera2d{..default()});
}
