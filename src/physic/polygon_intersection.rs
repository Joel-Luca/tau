use bevy::prelude::*;

use crate::physic::bounding_circle::BoundingCircle;
use crate::physic::bounding_polygon::BoundingPolygon;
use crate::physic::collision::IntersectsVolume;

impl IntersectsVolume<Self> for BoundingPolygon {
    fn intersects_volume(&self, other_polygon: &BoundingPolygon) -> bool {
        for (index, vertex) in self.vertices.iter().enumerate() {
            let next_vertex = &self.vertices[(index + 1) % self.vertices.len()];

            let edge = next_vertex - vertex;
            let axis = Vec2::new(-edge.y, edge.x).normalize();

            let (min_a, max_a) = self.project_vertices(axis);
            let (min_b, max_b) = other_polygon.project_vertices(axis);

            if min_a >= max_b || min_b >= max_a {
                return false;
            }
        }
        for (index, vertex) in other_polygon.vertices.iter().enumerate() {
            let next_vertex = &other_polygon.vertices[(index + 1) % other_polygon.vertices.len()];

            let edge = next_vertex - vertex;
            let axis = Vec2::new(-edge.y, edge.x).normalize();

            let (min_a, max_a) = self.project_vertices(axis);
            let (min_b, max_b) = other_polygon.project_vertices(axis);

            if min_a >= max_b || min_b >= max_a {
                return false;
            }
        }
        true
    }

    fn get_contact_vector(&self, other_polygon: &BoundingPolygon) -> Vec3 {
        let mut min: f32 = f32::MAX;
        let mut vec: Vec2 = Default::default();
        for (index, vertex) in self.vertices.iter().enumerate() {
            let next_vertex = &self.vertices[(index + 1) % self.vertices.len()];

            let (result, distance) =
                self.get_point_segment_distance(vertex, next_vertex, other_polygon);
            if distance < min {
                min = distance;
                vec = result;
            }
        }
        Vec3::new(vec.x, vec.y, 0.0)
    }
}

impl IntersectsVolume<BoundingCircle> for BoundingPolygon {
    fn intersects_volume(&self, circle: &BoundingCircle) -> bool {
        self.intersects_circle(circle)
    }

    fn get_contact_vector(&self, circle: &BoundingCircle) -> Vec3 {
        let mut min: f32 = f32::MAX;
        let mut vec: Vec2 = Default::default();
        for (index, vertex) in self.vertices.iter().enumerate() {
            let next_vertex = &self.vertices[(index + 1) % self.vertices.len()];

            let (result, distance) = circle.get_point_segment_distance(*vertex, *next_vertex);
            if distance < min {
                min = distance;
                vec = result;
            }
        }
        Vec3::new(vec.x, vec.y, 0.0)
    }
}
