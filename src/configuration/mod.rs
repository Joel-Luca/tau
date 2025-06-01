use bevy::prelude::*;

pub mod controls;
pub mod resolution;

pub struct ConfigurationPlugin;

impl Plugin for ConfigurationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((controls::ControlsPlugin, resolution::ResolutionPlugin))
            .add_systems(PreStartup, setup_configuration);
    }
}

#[derive(Resource)]
pub struct Configuration {
    pub bullet_speed: f32,
    pub move_speed: f32,
    pub rotation_speed: f32,
    pub shoot_interval: u128,
    pub spawn_protection: f32,
}

fn setup_configuration(mut commands: Commands) {
    commands.insert_resource(Configuration {
        bullet_speed: 500.,
        move_speed: 200.,
        rotation_speed: 3.,
        shoot_interval: 200,
        spawn_protection: 3.,
    })
}
