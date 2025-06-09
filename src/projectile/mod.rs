use crate::physic::collision::Collider;
use crate::player::Player;
use crate::projectile::bullet::Bullet;
use bevy::prelude::*;

pub mod bullet;
pub mod mine;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_projectile_collision);
    }
}

#[derive(Component)]
pub struct Projectile {}

fn check_projectile_collision(
    mut commands: Commands,
    player_query: Query<(Entity, &Collider), With<Player>>,
    projectile_query: Query<(Entity, &Collider), With<Projectile>>,
) {
    for (player, player_c) in player_query.iter() {
        for (bullet, projectile_) in projectile_query.iter() {
            if player_c.intersects(projectile_) {
                commands.entity(player).despawn();
                commands.entity(bullet).despawn();
            }
        }
    }
}
