use bevy::prelude::*;

pub mod enemy;
pub mod game;
pub mod player;
pub mod projectile;
pub mod resolution;

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins
                .set(WindowPlugin{
                    primary_window : Some(Window{
                        title : String::from("Hello World!"), 
                        position : WindowPosition::Centered(MonitorSelection::Primary),
                        ..Default::default()
                    }),
                    ..Default::default()
                })    
                .set(ImagePlugin::default_nearest()),
                game::GamePlugin
            ),
        )
        .run();
}
