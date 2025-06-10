use bevy::prelude::*;

pub mod bounce;
pub mod bounding_circle;
pub mod bounding_polygon;
pub mod bounding_volume;
pub mod circle_intersection;
pub mod collision;
pub mod polygon_intersection;
pub mod velocity;

pub struct PhysicPlugin;

impl Plugin for PhysicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((collision::CollisionPlugin, velocity::VelocityPlugin));
    }
}
