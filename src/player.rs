use bevy::prelude::*;

use crate::configuration::*;
use crate::configuration::controls::*;
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
pub struct Player{
    controls: Movement,
}


fn setup_player(mut commands : Commands, assets_server : Res<AssetServer>, resolution : Res<Resolution>, controls : Res<Controls>) {
    let player_texture = assets_server.load("player/tank_yellow.png");
    let spawn_location = Transform::from_translation(Vec3::new(0., 0., 0.)).with_scale(Vec3::splat(resolution.pixel_ratio));
    commands.spawn(
        (
            Player{ controls: controls.movement.clone() },
            TankBundle::new(spawn_location, Sprite::from_image(player_texture)),
        )
    );
}

fn move_player(
    mut query : Query<(&mut Transform, &Player)>,
    time : Res<Time>,
    keys : Res<ButtonInput<KeyCode>>,
    configuration : Res<Configuration>
)
{
    for (mut transform, player) in query.iter_mut() {
        let mut movement = 0.;
        let mut rotation = 0.;

        if keys.pressed(player.controls.forward) {
            rotation += 1.;
        }

        if keys.pressed(player.controls.backward) {
            rotation -= 1.;
        }

        if keys.pressed(player.controls.right) {
            movement += 1.;
        }

        if keys.pressed(player.controls.left) {
            movement -= 1.;
        }

        transform.rotate_z(rotation * configuration.rotation_speed * time.delta_secs());

        let direction = transform.rotation * Vec3::Y;
        let distance = movement * configuration.move_speed * time.delta_secs();

        transform.translation += direction * distance;
    }
}
