use bevy::prelude::*;

pub mod hide;

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(hide::HidePlugin);
    }
}
