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
    pub mine_visible_duration: u128,
    pub move_speed: f32,
    pub rotation_speed: f32,
    pub shoot_interval: u128,
    pub shuriken_bounce_count: u32,
    pub spawn_protection: f32,
    pub tank_mine_location: Vec3,
    pub tank_shoot_location: Vec3,
}

fn setup_configuration(mut commands: Commands) {
    commands.insert_resource(Configuration {
        bullet_speed: 1000.,
        mine_visible_duration: 5000,
        move_speed: 200.,
        rotation_speed: 3.,
        shoot_interval: 300,
        shuriken_bounce_count: 3,
        spawn_protection: 3.,
        tank_mine_location: Vec3::new(0., -55., 0.),
        tank_shoot_location: Vec3::new(0., 55., 0.),
    })
}
