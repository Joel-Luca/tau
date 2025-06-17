use bevy::prelude::*;
use rand::random_range;

use crate::configuration::resolution::Resolution;
use crate::physic::collision::collider::Collider;
use crate::physic::solid::Solid;

pub fn get_random_position(
    mut collider: Collider,
    solid: Query<&Collider, With<Solid>>,
    resolution: &Res<Resolution>,
) -> Vec3 {
    let mut pos: Option<Vec3> = None;

    while pos.is_none() {
        let l = (resolution.screen_dimensions.x.round() / 2.) as i32;
        let h = (resolution.screen_dimensions.y.round() / 2.) as i32;
        let x = random_range(-l..l) as f32;
        let y = random_range(-h..h) as f32;
        pos = Option::from(Vec3::new(x, y, 0.));
        let transform = Transform::from_translation(pos.unwrap());
        collider.update(&transform);
        for solid_c in solid.iter() {
            if collider.intersects(solid_c) {
                pos = None;
            }
        }
    }
    pos.unwrap()
}
