use bevy::prelude::*;

use crate::physic::collision::circle::CircleCollider;
use crate::physic::collision::intersection::IntersectCollider;

#[derive(Clone)]
pub struct PolygonCollider {
    pub relative_vertices: Box<[Vec2]>,
    pub vertices: Box<[Vec2]>,
    pub width: f32,
    pub height: f32,
}

impl PolygonCollider {
    pub fn new(vertices: Box<[Vec2]>) -> Self {
        let absolute_vertices = vertices.clone();
        let (width, height) = PolygonCollider::get_width_and_height(&vertices);
        Self {
            relative_vertices: vertices,
            vertices: absolute_vertices,
            width,
            height,
        }
    }

    pub fn update_vertices(&mut self, transform: &Transform) {
        for (index, vertex) in self.relative_vertices.iter().enumerate() {
            let vertex_as_vec_3 = Vec3::new(vertex.x, vertex.y, 0.);
            let new_vertex = transform.translation + transform.rotation * vertex_as_vec_3;
            self.vertices[index] = new_vertex.xy();
        }
    }

    pub fn project_vertices(&self, axis: Vec2) -> (f32, f32) {
        let mut min = f32::MAX;
        let mut max = f32::MIN;

        for vertex in self.vertices.iter() {
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

    pub fn intersects_circle(&self, circle: &CircleCollider) -> bool {
        for (index, vertex) in self.vertices.iter().enumerate() {
            let next_vertex = &self.vertices[(index + 1) % self.vertices.len()];

            let edge = next_vertex - vertex;
            let axis = Vec2::new(-edge.y, edge.x).normalize();

            let (min_a, max_a) = self.project_vertices(axis);
            let (min_b, max_b) = circle.project_circle(axis);

            if min_a >= max_b || min_b >= max_a {
                return false;
            }
        }

        let closest = self.get_closest_vertex(circle.center);
        let axis = (closest - circle.center).normalize();

        let (min_a, max_a) = self.project_vertices(axis);
        let (min_b, max_b) = circle.project_circle(axis);

        if min_a >= max_b || min_b >= max_a {
            return false;
        }

        true
    }

    pub fn get_point_segment_distance(&self, a: &Vec2, b: &Vec2, polygon: &Self) -> (Vec2, f32) {
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

    fn get_width_and_height(vertices: &Box<[Vec2]>) -> (f32, f32) {
        let mut max_x = f32::MIN;
        let mut min_x = f32::MAX;
        let mut max_y = f32::MIN;
        let mut min_y = f32::MAX;
        for vertex in vertices {
            if vertex.x > max_x {
                max_x = vertex.x
            } else if vertex.x < min_x {
                min_x = vertex.x
            }

            if vertex.y > max_y {
                max_y = vertex.y
            } else if vertex.y < min_y {
                min_y = vertex.y
            }
        }
        ((max_x - min_x).abs(), (max_y - min_y).abs())
    }
}

impl IntersectCollider<Self> for PolygonCollider {
    fn intersects(&self, other_polygon: &Self) -> bool {
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

    fn get_contact_vector(&self, other_polygon: &Self) -> Vec3 {
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

impl IntersectCollider<CircleCollider> for PolygonCollider {
    fn intersects(&self, circle: &CircleCollider) -> bool {
        self.intersects_circle(circle)
    }

    fn get_contact_vector(&self, circle: &CircleCollider) -> Vec3 {
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
