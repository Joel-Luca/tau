use bevy::prelude::*;


use crate::configuration::*;
use crate::configuration::resolution::*;
use crate::tank::*;

pub struct PlayerPlugin; 

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup_player)
        .add_systems(Update, move_player);
    }
}
 
#[derive(Component)]
pub struct Player{}


fn setup_player(mut commands : Commands, assets_server : Res<AssetServer>, resolution : Res<Resolution>) {
    let player_texture = assets_server.load("player/tank_yellow.png");
    let spawn_location = Transform::from_translation(Vec3::new(0., 0., 0.)).with_scale(Vec3::splat(resolution.pixel_ratio));
    commands.spawn(
        (
            Player{},
            TankBundle::new(spawn_location, Sprite::from_image(player_texture)),
        )
    );
}

fn move_player(
    mut transform_query : Query<&mut Transform, With<Player>>,
    time : Res<Time>,
    keys : Res<ButtonInput<KeyCode>>,
    configuration : Res<Configuration>
)
{
    let mut transform = transform_query.single_mut();

    let mut movement = 0.;
    let mut rotation = 0.;

    if keys.pressed(KeyCode::KeyA) {
        rotation += 1.;
    }

    if keys.pressed(KeyCode::KeyD) {
        rotation -= 1.;
    }

    if keys.pressed(KeyCode::KeyW) {
        movement += 1.;
    }

    if keys.pressed(KeyCode::KeyS) {
        movement -= 1.;
    }

    transform.rotate_z(rotation * configuration.rotation_speed * time.delta_secs());

    let direction = transform.rotation * Vec3::Y;
    let distance = movement * configuration.move_speed * time.delta_secs();

    transform.translation += direction * distance;
}
