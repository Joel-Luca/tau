use crate::configuration::resolution::Resolution;
use crate::physic::bounding_polygon::BoundingPolygon;
use crate::physic::collision::{Collider, Intersects};
use crate::weapon::Weapon;
use bevy::prelude::*;

#[derive(Component)]
pub struct Chest {
    weapon: Weapon,
}

#[derive(Bundle)]
pub struct ChestBundle {
    chest: Chest,
    collider: Collider,
    intersects: Intersects,
    sprite: Sprite,
    transform: Transform,
}

impl ChestBundle {
    pub fn new(
        position: Vec3,
        assets_server: &Res<AssetServer>,
        resolution: &Res<Resolution>,
    ) -> ChestBundle {
        let weapon = Weapon::random();
        let chest_texture = assets_server.load(weapon.get_asset_name());
        let collider = BoundingPolygon::new(Box::new([]));
        ChestBundle {
            chest: Chest { weapon },
            collider: Collider::Polygon(collider),
            intersects: Intersects::default(),
            sprite: Sprite::from_image(chest_texture),
            transform: Transform::from_translation(position)
                .with_scale(Vec3::splat(resolution.chest_pixel_ratio)),
        }
    }
}
