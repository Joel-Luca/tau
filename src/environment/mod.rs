use bevy::prelude::*;

pub mod chest;
mod random;
pub mod wall;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((chest::ChestPlugin, wall::WallPlugin));
    }
}
