use bevy::prelude::*;

pub mod wall;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(wall::WallPlugin);
    }
}
