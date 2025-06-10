use bevy::prelude::*;
use std::time::SystemTime;

use crate::ability::hide::Hide;
use crate::configuration::Configuration;
use crate::configuration::resolution::Resolution;
use crate::physic::bounding_circle::BoundingCircle;
use crate::physic::collision::{Collider, Intersects};
use crate::projectile::Projectile;

#[derive(Component)]
pub struct Mine {}

#[derive(Bundle)]
pub struct MineBundle {
    collider: Collider,
    hide: Hide,
    intersects: Intersects,
    mine: Mine,
    projectile: Projectile,
    sprite: Sprite,
    transform: Transform,
}

impl MineBundle {
    pub fn new(
        tank_position: &Transform,
        spawn_time: SystemTime,
        assets_server: &Res<AssetServer>,
        configuration: &Res<Configuration>,
        resolution: &Res<Resolution>,
    ) -> MineBundle {
        let mine_texture = assets_server.load("ammunition/mine.png");
        let collider = BoundingCircle {
            radius: 20.,
            center: tank_position.translation.xy(),
        };
        let position: Vec3 =
            tank_position.translation + tank_position.rotation * configuration.tank_mine_location;
        let spawn_location = Transform::from_translation(position)
            .with_scale(Vec3::splat(resolution.mine_pixel_ratio));
        MineBundle {
            collider: Collider::Circle(collider),
            hide: Hide {
                spawn_time,
                visible_duration: configuration.mine_visible_duration,
            },
            intersects: Intersects::default(),
            mine: Mine {},
            projectile: Projectile {},
            sprite: Sprite::from_image(mine_texture),
            transform: spawn_location,
        }
    }
}
