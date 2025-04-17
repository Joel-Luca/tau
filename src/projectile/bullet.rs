use bevy::prelude::*;


use crate::physic::velocity::*;
use crate::projectile::*;

pub struct BulletPlugin; 

impl Plugin for BulletPlugin{
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct Bullet{}

#[derive(Bundle)]
pub struct BulletBundle {
    bullet: Bullet,
    projectile: Projectile,
    velocity: Velocity,
}
