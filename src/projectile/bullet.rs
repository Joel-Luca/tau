use bevy::prelude::*;


use crate::physic::velocity::*;
use crate::projectile::*;

#[derive(Component)]
pub struct Bullet{}

#[derive(Bundle)]
pub struct BulletBundle {
    bullet: Bullet,
    projectile: Projectile,
    velocity: Velocity,
}
