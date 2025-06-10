use bevy::prelude::*;

pub struct ResolutionPlugin;

impl Plugin for ResolutionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_resolution);
    }
}

#[derive(Resource)]
pub struct Resolution {
    pub screen_dimensions: Vec2,
    pub ammunition_pixel_ratio: f32,
    pub mine_pixel_ratio: f32,
    pub projectile_pixel_ratio: f32,
    pub shuriken_pixel_ratio: f32,
    pub tank_pixel_ratio: f32,
}

fn setup_resolution(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.single();
    commands.insert_resource(Resolution {
        screen_dimensions: Vec2::new(window.width(), window.height()),
        ammunition_pixel_ratio: 0.05,
        mine_pixel_ratio: 0.05,
        projectile_pixel_ratio: 0.01,
        shuriken_pixel_ratio: 0.05,
        tank_pixel_ratio: 0.1,
    })
}
