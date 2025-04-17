use bevy::prelude::*;

pub struct ConfigurationPlugin;

impl Plugin for ConfigurationPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_configuration);
    } 
}

#[derive(Resource)]
pub struct Configuration{
    pub move_speed : f32,
    pub rotation_speed : f32,
    pub spawn_protection : f32,
}

fn setup_configuration(mut commands : Commands) {
    commands.insert_resource(Configuration{
        move_speed : 200.,
        rotation_speed: 3.,
        spawn_protection: 3.,
    })
}

