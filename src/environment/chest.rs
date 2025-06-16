use bevy::prelude::*;

use crate::configuration::resolution::Resolution;
use crate::physic::collision::Collision;
use crate::physic::collision::collider::Collider;
use crate::physic::collision::polygon::PolygonCollider;
use crate::weapon::Weapon;

#[derive(Component)]
pub struct Chest {
    weapon: Weapon,
}

#[derive(Bundle)]
pub struct ChestBundle {
    chest: Chest,
    collider: Collision,
    sprite: Sprite,
}

impl ChestBundle {
    pub fn new(
        position: Vec3,
        assets_server: &Res<AssetServer>,
        resolution: &Res<Resolution>,
    ) -> ChestBundle {
        let weapon = Weapon::random();
        let chest_texture = assets_server.load(weapon.get_asset_name());
        let collider = PolygonCollider::new(Box::new([]));
        let spwan_location = Transform::from_translation(position)
            .with_scale(Vec3::splat(resolution.chest_pixel_ratio));
        ChestBundle {
            chest: Chest { weapon },
            collider: Collision::new(Collider::Polygon(collider), spwan_location),
            sprite: Sprite::from_image(chest_texture),
        }
    }
}
