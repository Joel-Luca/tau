use std::time::SystemTime;

use bevy::prelude::*;

use crate::ability::hide::Hide;
use crate::configuration::resolution::Resolution;
use crate::configuration::Configuration;
use crate::physic::collision::circle::CircleCollider;
use crate::physic::collision::collider::Collider;
use crate::physic::collision::Collision;
use crate::projectile::Projectile;

#[derive(Component)]
pub struct Mine {}

#[derive(Bundle)]
pub struct MineBundle {
    collider: Collision,
    hide: Hide,
    mine: Mine,
    projectile: Projectile,
    sprite: Sprite,
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
        let collider = CircleCollider::new(20., tank_position.translation.xy());
        let position: Vec3 =
            tank_position.translation + tank_position.rotation * configuration.tank_mine_location;
        let spawn_location = Transform::from_translation(position)
            .with_scale(Vec3::splat(resolution.mine_pixel_ratio));
        MineBundle {
            collider: Collision::new(Collider::Circle(collider), spawn_location),
            hide: Hide {
                spawn_time,
                visible_duration: configuration.mine_visible_duration,
            },
            mine: Mine {},
            projectile: Projectile {},
            sprite: Sprite::from_image(mine_texture),
        }
    }
}
