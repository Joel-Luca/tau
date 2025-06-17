use std::time::SystemTime;

use bevy::prelude::*;

use crate::configuration::controls::Controls;
use crate::configuration::controls::Movement;
use crate::configuration::resolution::Resolution;
use crate::configuration::Configuration;
use crate::tank::TankBundle;
use crate::weapon::Weapon;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, (move_player, shoot).chain());
    }
}

#[derive(Component)]
pub struct Player {
    controls: Movement,
    last_shot: SystemTime,
    last_pos: Vec3,
}

impl Player {
    pub fn new(controls: Movement) -> Player {
        Player {
            controls,
            last_shot: SystemTime::now(),
            last_pos: Vec3::ZERO,
        }
    }

    pub fn set_to_last_pos(&self, transform: &mut Transform) {
        transform.translation = self.last_pos;
    }
}

fn setup_player(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    resolution: Res<Resolution>,
    controls: Res<Controls>,
) {
    let player_1_texture = assets_server.load("player/tank_yellow.png");
    let player_2_texture = assets_server.load("player/tank_pink.png");
    let spawn_location_1 = Transform::from_translation(Vec3::new(0., 0., 0.))
        .with_scale(Vec3::splat(resolution.tank_pixel_ratio));
    let spawn_location_2 = Transform::from_translation(Vec3::new(-100., 0., 0.))
        .with_scale(Vec3::splat(resolution.tank_pixel_ratio));
    commands.spawn((
        Player::new(controls.movement.clone()),
        TankBundle::new(spawn_location_1, Sprite::from_image(player_1_texture)),
    ));

    commands.spawn((
        Player::new(controls.second_movement.clone()),
        TankBundle::new(spawn_location_2, Sprite::from_image(player_2_texture)),
    ));
}

fn move_player(
    mut query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    configuration: Res<Configuration>,
) {
    for (mut transform, mut player) in query.iter_mut() {
        let mut movement = 0.;
        let mut rotation = 0.;

        if keys.pressed(player.controls.left) {
            rotation += 1.;
        }

        if keys.pressed(player.controls.right) {
            rotation -= 1.;
        }

        if keys.pressed(player.controls.forward) {
            movement += 1.;
        }

        if keys.pressed(player.controls.backward) {
            movement -= 1.;
        }

        transform.rotate_z(rotation * configuration.rotation_speed * time.delta_secs());

        let direction = transform.rotation * Vec3::Y;
        let distance = movement * configuration.move_speed * time.delta_secs();

        player.last_pos = transform.translation;
        transform.translation += direction * distance;
    }
}

fn shoot(
    mut query: Query<(&mut Player, &Transform, &Weapon)>,
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    configuration: Res<Configuration>,
    resolution: Res<Resolution>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (mut player, transform, weapon) in query.iter_mut() {
        let duration = SystemTime::now().duration_since(player.last_shot).unwrap();
        if keys.pressed(player.controls.shoot)
            && duration.as_millis() > configuration.shoot_interval
        {
            player.last_shot = SystemTime::now();
            weapon.shoot(
                transform,
                &assets_server,
                &mut commands,
                &configuration,
                &resolution,
            );
        }
    }
}
