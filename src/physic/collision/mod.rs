use bevy::prelude::*;

use crate::physic::collision::collider::update_colliders;
use crate::physic::collision::collider::Collider;
use crate::physic::collision::intersection::Intersection;
use crate::physic::solid::Solid;
use crate::player::Player;

pub mod circle;
pub mod collider;
mod intersection;
pub mod polygon;
mod visibility;
mod volume;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(visibility::VisibilityPlugin).add_systems(
            PostUpdate,
            (update_colliders, check_player_collision).chain(),
        );
    }
}

#[derive(Bundle)]
pub struct Collision {
    collider: Collider,
    intersection: Intersection,
    transform: Transform,
}

impl Collision {
    pub fn new(mut collider: Collider, transform: Transform) -> Self {
        collider.update(&transform);
        Self {
            collider,
            intersection: Intersection::default(),
            transform,
        }
    }
}

fn check_player_collision(
    mut player_query: Query<(Entity, &mut Transform, &Player, &Collider)>,
    solid_query: Query<(Entity, &Collider), With<Solid>>,
) {
    for (player_entity, mut transform, player, player_c) in player_query.iter_mut() {
        for (solid_entity, solid_c) in solid_query.iter() {
            if player_entity.index() != solid_entity.index() && player_c.intersects(solid_c) {
                player.set_to_last_pos(&mut transform);
                return;
            }
        }
    }
}
