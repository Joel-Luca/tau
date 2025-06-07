use crate::configuration::Configuration;
use crate::physic::collision::*;
use crate::physic::velocity::*;
use crate::projectile::*;

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
        position: Transform,
        direction: Vec3,
        assets_server: &Res<AssetServer>,
        configuration: &Res<Configuration>,
    ) -> BulletBundle {
        let bullet_texture = assets_server.load("ammunition/bullet.png");
        let collider = BoundingCircle {
            radius: 5.,
            center: position.translation.xy(),
        };
        let velocity = configuration.bullet_speed * direction;
        BulletBundle {
            bullet: Bullet {},
            collider: Collider::Circle(collider),
            intersects: Intersects::default(),
            projectile: Projectile {},
            sprite: Sprite::from_image(bullet_texture),
            transform: position,
            velocity: Velocity(velocity),
        }
    }
}
