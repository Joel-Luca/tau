use bevy::prelude::*;

use crate::physic::collision::intersection::IntersectCollider;
use crate::physic::collision::polygon::PolygonCollider;

pub struct CircleCollider {
    pub radius: f32,
    pub center: Vec2,
    pub width: f32,
    pub height: f32,
}

impl CircleCollider {
    pub fn update_center(&mut self, transform: &Transform) {
        self.center = transform.translation.xy();
    }

    pub(crate) fn project_circle(&self, axis: Vec2) -> (f32, f32) {
        let direction = axis.normalize();
        let vector = self.radius * direction;
        let p1 = self.center + vector;
        let p2 = self.center - vector;

        let mut min = p1.dot(axis);
        let mut max = p2.dot(axis);

        if min > max {
            let temp = min;
            min = max;
            max = temp;
        }

        (min, max)
    }

    pub fn get_point_segment_distance(&self, a: Vec2, b: Vec2) -> (Vec2, f32) {
        let edge = b - a;
        let dist = self.center - a;
        let proj = edge.dot(dist);
        let len = edge.length_squared();
        let distance = proj / len;

        let result;
        if distance <= 0. {
            result = a;
        } else if distance >= 1. {
            result = b;
        } else {
            result = a + edge * distance;
        }
        (result, self.center.distance(result))
    }
}

impl IntersectCollider<Self> for CircleCollider {
    fn intersects(&self, circle: &Self) -> bool {
        let distance: f32 = self.center.distance(circle.center);
        let radii: f32 = self.radius + circle.radius;
        distance < radii
    }

    fn get_contact_vector(&self, circle: &Self) -> Vec3 {
        let ab = circle.center - self.center;
        let vec = self.center + ab.normalize() * self.radius;
        Vec3::new(-vec.y, vec.x, 0.)
    }
}

impl IntersectCollider<PolygonCollider> for CircleCollider {
    fn intersects(&self, polygon: &PolygonCollider) -> bool {
        polygon.intersects_circle(self)
    }

    fn get_contact_vector(&self, polygon: &PolygonCollider) -> Vec3 {
        let mut min: f32 = f32::MAX;
        let mut vec: Vec2 = Default::default();
        for (index, vertex) in polygon.vertices.iter().enumerate() {
            let next_vertex = &polygon.vertices[(index + 1) % polygon.vertices.len()];

            let (_, distance) = self.get_point_segment_distance(*vertex, *next_vertex);
            if distance < min {
                min = distance;
                vec = next_vertex - vertex;
            }
        }
        Vec3::new(vec.x, vec.y, 0.)
    }
}
