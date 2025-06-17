use std::time::SystemTime;

use bevy::prelude::*;
use rand::random_range;

use crate::configuration::resolution::Resolution;
use crate::configuration::Configuration;
use crate::projectile::bullet::BulletBundle;
use crate::projectile::mine::MineBundle;
use crate::projectile::shuriken::ShurikenBundle;

#[derive(Component, Clone)]
pub enum Weapon {
    Bullet,
    Mine,
    Shuriken,
}

impl Weapon {
    pub fn random() -> Weapon {
        let weapons = [Weapon::Bullet, Weapon::Mine, Weapon::Shuriken];
        let index = random_range(0..weapons.len());
        weapons[index].to_owned()
    }

    pub fn default() -> Weapon {
        Weapon::Shuriken
    }

    pub fn get_asset_name(&self) -> String {
        let weapon_str;
        match &self {
            Weapon::Bullet => weapon_str = "mg",
            Weapon::Mine => weapon_str = "mine",
            Weapon::Shuriken => weapon_str = "shuriken",
        }
        "environment/chest_".to_string() + weapon_str + ".png"
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
                commands.spawn(BulletBundle::new(
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
                    transform,
                    assets_server,
                    configuration,
                    resolution,
                ));
            }
        };
    }
}
