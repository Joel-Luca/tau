use bevy::prelude::*;

use crate::configuration::resolution::Resolution;
use crate::configuration::Configuration;
use crate::physic::bounce::Bounce;
use crate::physic::collision::circle::CircleCollider;
use crate::physic::collision::collider::Collider;
use crate::physic::collision::Collision;
use crate::physic::velocity::Velocity;
use crate::projectile::Projectile;

#[derive(Component)]
pub struct Shuriken {}

#[derive(Bundle)]
pub struct ShurikenBundle {
    bounce: Bounce,
    collider: Collision,
    projectile: Projectile,
    shuriken: Shuriken,
    sprite: Sprite,
    velocity: Velocity,
}

impl ShurikenBundle {
    pub fn new(
        tank_position: &Transform,
        assets_server: &Res<AssetServer>,
        configuration: &Res<Configuration>,
        resolution: &Res<Resolution>,
    ) -> ShurikenBundle {
        let direction = tank_position.rotation * Vec3::Y;
        let shuriken_texture = assets_server.load("ammunition/shuriken.png");
        let collider = CircleCollider {
            radius: 5.,
            center: tank_position.translation.xy(),
        };
        let position: Vec3 =
            tank_position.translation + tank_position.rotation * configuration.tank_shoot_location;
        let spawn_location = Transform::from_translation(position)
            .with_scale(Vec3::splat(resolution.shuriken_pixel_ratio));
        let velocity = configuration.shuriken_speed * direction;
        ShurikenBundle {
            bounce: Bounce {
                bounce_count: configuration.shuriken_bounce_count,
                last_bounce: Entity::PLACEHOLDER,
            },
            collider: Collision::new(Collider::Circle(collider), spawn_location),
            projectile: Projectile {},
            shuriken: Shuriken {},
            sprite: Sprite::from_image(shuriken_texture),
            velocity: Velocity(velocity),
        }
    }
}
