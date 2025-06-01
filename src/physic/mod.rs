use bevy::prelude::*;

pub mod collision;
pub mod velocity;

pub struct PhysicPlugin;

impl Plugin for PhysicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((collision::CollisionPlugin, velocity::VelocityPlugin));
    }
}
