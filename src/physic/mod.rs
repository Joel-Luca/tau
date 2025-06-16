use bevy::prelude::*;

pub mod bounce;
pub mod collision;
pub mod solid;
pub mod velocity;

pub struct PhysicPlugin;

impl Plugin for PhysicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            bounce::BouncePlugin,
            collision::CollisionPlugin,
            velocity::VelocityPlugin,
        ));
    }
}
