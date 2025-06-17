use bevy::prelude::*;

use crate::configuration::resolution::Resolution;
use crate::configuration::Configuration;
use crate::physic::collision::circle::CircleCollider;
use crate::physic::collision::collider::Collider;
use crate::physic::collision::Collision;
use crate::physic::velocity::Velocity;
use crate::projectile::Projectile;

#[derive(Component)]
pub struct Bullet {}

#[derive(Bundle)]
pub struct BulletBundle {
    bullet: Bullet,
    collider: Collision,
    projectile: Projectile,
    sprite: Sprite,
    velocity: Velocity,
}

impl BulletBundle {
    pub fn new(
        tank_position: &Transform,
        assets_server: &Res<AssetServer>,
        configuration: &Res<Configuration>,
        resolution: &Res<Resolution>,
    ) -> BulletBundle {
        let direction = tank_position.rotation * Vec3::Y;
        let bullet_texture = assets_server.load("ammunition/bullet.png");
        let collider = CircleCollider {
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
            collider: Collision::new(Collider::Circle(collider), spawn_location),
            projectile: Projectile {},
            sprite: Sprite::from_image(bullet_texture),
            velocity: Velocity(velocity),
        }
    }
}
