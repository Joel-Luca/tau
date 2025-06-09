use bevy::prelude::*;
use std::time::SystemTime;

use crate::configuration::resolution::*;
use crate::configuration::*;
use crate::projectile::bullet::*;
use crate::projectile::mine::*;

#[derive(Component)]
pub enum Weapon {
    Bullet,
    Mine,
}

impl Weapon {
    pub fn default() -> Weapon {
        Weapon::Bullet
    }

    pub fn shoot(
        &self,
        transform: &Transform,
        assets_server: &Res<AssetServer>,
        commands: &mut Commands,
        configuration: &Res<Configuration>,
        resolution: &Res<Resolution>,
    ) {
        match self {
            &Weapon::Bullet => {
                let direction = transform.rotation * Vec3::Y;
                commands.spawn(BulletBundle::new(
                    direction,
                    transform,
                    assets_server,
                    configuration,
                    resolution,
                ));
            }
            &Weapon::Mine => {
                commands.spawn(MineBundle::new(
                    transform,
                    SystemTime::now(),
                    assets_server,
                    configuration,
                    resolution,
                ));
            }
        };
    }
}
