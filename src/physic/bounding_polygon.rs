use bevy::prelude::*;

use crate::physic::bounding_circle::BoundingCircle;

pub struct BoundingPolygon {
    pub relative_vertices: Box<[Vec2]>,
    pub vertices: Box<[Vec2]>,
}

impl BoundingPolygon {
    pub fn new(vertices: Box<[Vec2]>) -> BoundingPolygon {
        let absolute_vertices = vertices.clone();
        BoundingPolygon {
            relative_vertices: vertices,
            vertices: absolute_vertices,
        }
    }

    pub fn update_vertices(&mut self, transform: &Transform) {
        for (index, vertex) in self.relative_vertices.iter().enumerate() {
            self.vertices[index].x = transform.translation.x + vertex.x;
            self.vertices[index].y = transform.translation.y + vertex.y;
        }
    }

    pub(crate) fn project_vertices(&self, vertices: &Box<[Vec2]>, axis: Vec2) -> (f32, f32) {
        let mut min = f32::MAX;
        let mut max = f32::MIN;

        for vertex in vertices {
            let projection = vertex.dot(axis);

            if projection < min {
                min = projection;
            }

            if projection > max {
                max = projection;
            }
        }

        (min, max)
    }

    pub fn get_closest_vertex(&self, point: Vec2) -> Vec2 {
        let mut closest: Vec2 = Default::default();
        let mut min: f32 = f32::MAX;

        for vertex in self.vertices.iter() {
            let distance = vertex.distance(point);

            if distance < min {
                min = distance;
                closest = vertex.clone();
            }
        }

        closest
    }

    pub fn intersects_circle(&self, circle: &BoundingCircle) -> bool {
        for (index, vertex) in self.vertices.iter().enumerate() {
            let next_vertex = &self.vertices[(index + 1) % self.vertices.len()];

            let edge = next_vertex - vertex;
            let axis = Vec2::new(-edge.y, edge.x).normalize();

            let (min_a, max_a) = self.project_vertices(&self.vertices, axis);
            let (min_b, max_b) = circle.project_circle(axis);

            if min_a >= max_b || min_b >= max_a {
                return false;
            }
        }

        let closest = self.get_closest_vertex(circle.center);
        let axis = (closest - circle.center).normalize();

        let (min_a, max_a) = self.project_vertices(&self.vertices, axis);
        let (min_b, max_b) = circle.project_circle(axis);

        if min_a >= max_b || min_b >= max_a {
            return false;
        }

        true
    }

    pub fn get_point_segment_distance(
        &self,
        a: &Vec2,
        b: &Vec2,
        polygon: &BoundingPolygon,
    ) -> (Vec2, f32) {
        let edge = b - a;

        let mut min: f32 = f32::MAX;
        let mut vec: Vec2 = Default::default();
        for (index, vertex) in polygon.vertices.iter().enumerate() {
            let next_vertex = &polygon.vertices[(index + 1) % polygon.vertices.len()];

            let other_edge = next_vertex - vertex;

            let distance = edge.distance_squared(other_edge);
            if distance < min {
                min = distance;
                vec = other_edge;
            }
        }
        (vec, min)
    }
}
