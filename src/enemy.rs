use bevy::prelude::*;

use crate::configuration::resolution::Resolution;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemies)
            .add_systems(Update, update_enemies);
    }
}

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct Enemy {
    pub dead: bool,
}

fn setup_enemies(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    resolution: Res<Resolution>,
) {
    /*
    let enemy_texture = assets_server.load("player/tank_pink.png");
    let spawn_location = Transform::from_translation(Vec3::new(0., 0., 0.)).with_scale(Vec3::splat(resolution.pixel_ratio));
    commands.spawn(
        (
            Enemy{
                dead : false,
            },
            TankBundle::new(spawn_location, Sprite::from_image(enemy_texture)),
        )
    );
    */
}

fn update_enemies(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Enemy, &mut Visibility), Without<Dead>>,
) {
    for (entity, enemy, mut visibility) in enemy_query.iter_mut() {
        if enemy.dead {
            commands.entity(entity).insert(Dead {});
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }
    }
}
