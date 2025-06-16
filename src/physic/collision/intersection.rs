use bevy::prelude::*;

use crate::physic::collision::collider::Collider;
use crate::physic::collision::volume::ColliderVolume;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Intersection(bool);

pub trait IntersectCollider<Volume: ColliderVolume + ?Sized> {
    fn intersects(&self, volume: &Volume) -> bool;
    fn get_contact_vector(&self, volume: &Volume) -> Vec3;
}

pub fn update_intersection(
    mut collider_query: Query<(Entity, &Collider, &mut Intersection)>,
    possible_collisions: Query<(Entity, &Collider)>,
) {
    for (entity, collider, mut intersects) in collider_query.iter_mut() {
        let mut collied: bool = false;
        for (other_entity, other_collider) in possible_collisions.iter() {
            if entity.index() == other_entity.index() {
                continue;
            }

            if collider.intersects(other_collider) {
                collied = true;
            }
        }
        **intersects = collied;
    }
}
