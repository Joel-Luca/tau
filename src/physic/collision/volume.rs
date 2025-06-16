use crate::physic::collision::circle::CircleCollider;
use crate::physic::collision::polygon::PolygonCollider;

pub trait ColliderVolume {}

impl ColliderVolume for CircleCollider {}

impl ColliderVolume for PolygonCollider {}
