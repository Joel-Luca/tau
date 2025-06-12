use bevy::prelude::*;

use crate::physic::bounding_circle::BoundingCircle;
use crate::physic::bounding_polygon::BoundingPolygon;
use crate::physic::collision::IntersectsVolume;

impl IntersectsVolume<Self> for BoundingCircle {
    fn intersects_volume(&self, circle: &BoundingCircle) -> bool {
        let distance: f32 = self.center.distance(circle.center);
        let radii: f32 = self.radius + circle.radius;
        distance < radii
    }

    fn get_contact_vector(&self, circle: &BoundingCircle) -> Vec3 {
        let ab = circle.center - self.center;
        let vec = self.center + ab.normalize() * self.radius;
        Vec3::new(-vec.y, vec.x, 0.)
    }
}

impl IntersectsVolume<BoundingPolygon> for BoundingCircle {
    fn intersects_volume(&self, polygon: &BoundingPolygon) -> bool {
        polygon.intersects_circle(self)
    }

    fn get_contact_vector(&self, polygon: &BoundingPolygon) -> Vec3 {
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
