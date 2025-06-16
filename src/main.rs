use bevy::prelude::*;

pub mod ability;
pub mod configuration;
pub mod enemy;
pub mod environment;
pub mod game;
pub mod pathfinding;
pub mod physic;
pub mod player;
pub mod projectile;
pub mod tank;
pub mod weapon;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Tau"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            game::GamePlugin,
        ))
        .run();
}
