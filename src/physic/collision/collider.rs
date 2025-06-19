use bevy::prelude::*;

use crate::physic::collision::circle::CircleCollider;
use crate::physic::collision::intersection::IntersectCollider;
use crate::physic::collision::polygon::PolygonCollider;

#[derive(Component, Clone)]
pub enum Collider {
    Polygon(PolygonCollider),
    Circle(CircleCollider),
}

impl Collider {
    pub fn intersects(&self, other: &Self) -> bool {
        match &*self {
            Self::Polygon(a) => match &*other {
                Self::Polygon(collided_a) => a.intersects(collided_a),
                Self::Circle(collided_c) => a.intersects(collided_c),
            },
            Self::Circle(c) => match &*other {
                Self::Polygon(collided_a) => c.intersects(collided_a),
                Self::Circle(collided_c) => c.intersects(collided_c),
            },
        }
    }

    pub fn get_contact_vector(&self, other: &Self) -> Vec3 {
        match &*self {
            Self::Polygon(a) => match &*other {
                Self::Polygon(collided_a) => a.get_contact_vector(collided_a),
                Self::Circle(collided_c) => a.get_contact_vector(collided_c),
            },
            Self::Circle(c) => match &*other {
                Self::Polygon(collided_a) => c.get_contact_vector(collided_a),
                Self::Circle(collided_c) => c.get_contact_vector(collided_c),
            },
        }
    }

    pub fn update(&mut self, transform: &Transform) {
        match *self {
            Collider::Polygon(ref mut polygon) => {
                polygon.update_vertices(transform);
            }
            Collider::Circle(ref mut circle) => {
                circle.update_center(transform);
            }
        }
    }

    pub fn width(&self) -> f32 {
        match self {
            Collider::Polygon(polygon) => polygon.width,
            Collider::Circle(circle) => circle.width,
        }
    }

    pub fn height(&self) -> f32 {
        match self {
            Collider::Polygon(polygon) => polygon.height,
            Collider::Circle(circle) => circle.height,
        }
    }
}

pub fn update_colliders(mut query: Query<(&mut Collider, &Transform)>) {
    for (mut collider, transform) in query.iter_mut() {
        collider.update(&transform);
    }
}
