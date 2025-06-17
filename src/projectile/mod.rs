use bevy::prelude::*;

use crate::physic::bounce::Bounce;
use crate::physic::collision::collider::Collider;
use crate::physic::solid::Solid;
use crate::player::Player;

pub mod bullet;
pub mod mine;
pub mod shuriken;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (check_player_collision, check_solid_collision));
    }
}

#[derive(Component)]
pub struct Projectile {}

fn check_player_collision(
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

fn check_solid_collision(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Collider), (With<Projectile>, Without<Bounce>)>,
    solid_query: Query<&Collider, With<Solid>>,
) {
    for (projectile, projectile_c) in projectile_query.iter() {
        for wall_c in solid_query.iter() {
            if projectile_c.intersects(wall_c) {
                commands.entity(projectile).despawn();
            }
        }
    }
}
