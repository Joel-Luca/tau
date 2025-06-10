use crate::physic::bounding_circle::BoundingCircle;
use crate::physic::bounding_polygon::BoundingPolygon;

pub trait BoundingVolume {}

impl BoundingVolume for BoundingPolygon {}

impl BoundingVolume for BoundingCircle {}
