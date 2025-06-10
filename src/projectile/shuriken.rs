use bevy::prelude::*;

use crate::configuration::resolution::Resolution;
use crate::configuration::Configuration;
use crate::physic::bounce::Bounce;
use crate::physic::bounding_circle::BoundingCircle;
use crate::physic::collision::{Collider, Intersects};
use crate::physic::velocity::Velocity;
use crate::projectile::Projectile;

#[derive(Component)]
pub struct Shuriken {}

#[derive(Bundle)]
pub struct ShurikenBundle {
    bounce: Bounce,
    collider: Collider,
    intersects: Intersects,
    projectile: Projectile,
    shuriken: Shuriken,
    sprite: Sprite,
    transform: Transform,
    velocity: Velocity,
}

impl ShurikenBundle {
    pub fn new(
        direction: Vec3,
        tank_position: &Transform,
        assets_server: &Res<AssetServer>,
        configuration: &Res<Configuration>,
        resolution: &Res<Resolution>,
    ) -> ShurikenBundle {
        let shuriken_texture = assets_server.load("ammunition/shuriken.png");
        let collider = BoundingCircle {
            radius: 5.,
            center: tank_position.translation.xy(),
        };
        let position: Vec3 =
            tank_position.translation + tank_position.rotation * configuration.tank_shoot_location;
        let spawn_location = Transform::from_translation(position)
            .with_scale(Vec3::splat(resolution.projectile_pixel_ratio));
        let velocity = configuration.shuriken_speed * direction;
        ShurikenBundle {
            bounce: Bounce { bounce_count: configuration.shuriken_bounce_count },
            collider: Collider::Circle(collider),
            intersects: Intersects::default(),
            projectile: Projectile {},
            shuriken: Shuriken {},
            sprite: Sprite::from_image(shuriken_texture),
            transform: spawn_location,
            velocity: Velocity(velocity),
        }
    }
}
