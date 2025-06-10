use bevy::prelude::*;
use std::time::SystemTime;

use crate::configuration::Configuration;
use crate::configuration::resolution::Resolution;
use crate::projectile::bullet::BulletBundle;
use crate::projectile::mine::MineBundle;
use crate::projectile::shuriken::ShurikenBundle;

#[derive(Component)]
pub enum Weapon {
    Bullet,
    Mine,
    Shuriken,
}

impl Weapon {
    pub fn default() -> Weapon {
        Weapon::Shuriken
    }

    pub fn shoot(
        &self,
        transform: &Transform,
        assets_server: &Res<AssetServer>,
        commands: &mut Commands,
        configuration: &Res<Configuration>,
        resolution: &Res<Resolution>,
    ) {
        let direction = transform.rotation * Vec3::Y;
        match self {
            &Weapon::Bullet => {
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
            &Weapon::Shuriken => {
                commands.spawn(ShurikenBundle::new(
                    direction,
                    transform,
                    assets_server,
                    configuration,
                    resolution,
                ));
            }
        };
    }
}
