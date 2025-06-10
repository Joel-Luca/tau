use bevy::prelude::*;

use crate::configuration::Configuration;
use crate::configuration::resolution::Resolution;
use crate::physic::bounding_circle::BoundingCircle;
use crate::physic::collision::{Collider, Intersects};
use crate::physic::velocity::Velocity;
use crate::projectile::Projectile;

#[derive(Component)]
pub struct Bullet {}

#[derive(Bundle)]
pub struct BulletBundle {
    bullet: Bullet,
    collider: Collider,
    intersects: Intersects,
    projectile: Projectile,
    sprite: Sprite,
    transform: Transform,
    velocity: Velocity,
}

impl BulletBundle {
    pub fn new(
        direction: Vec3,
        tank_position: &Transform,
        assets_server: &Res<AssetServer>,
        configuration: &Res<Configuration>,
        resolution: &Res<Resolution>,
    ) -> BulletBundle {
        let bullet_texture = assets_server.load("ammunition/bullet.png");
        let collider = BoundingCircle {
            radius: 5.,
            center: tank_position.translation.xy(),
        };
        let position: Vec3 =
            tank_position.translation + tank_position.rotation * configuration.tank_shoot_location;
        let spawn_location = Transform::from_translation(position)
            .with_scale(Vec3::splat(resolution.projectile_pixel_ratio));
        let velocity = configuration.bullet_speed * direction;
        BulletBundle {
            bullet: Bullet {},
            collider: Collider::Circle(collider),
            intersects: Intersects::default(),
            projectile: Projectile {},
            sprite: Sprite::from_image(bullet_texture),
            transform: spawn_location,
            velocity: Velocity(velocity),
        }
    }
}
