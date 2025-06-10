use bevy::prelude::*;

pub struct BoundingCircle {
    pub radius: f32,
    pub center: Vec2,
}

impl BoundingCircle {
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
