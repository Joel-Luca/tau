use bevy::prelude::*;

use crate::configuration::resolution::Resolution;
use crate::environment::random::get_random_position;
use crate::physic::collision::collider::Collider;
use crate::physic::collision::polygon::PolygonCollider;
use crate::physic::collision::Collision;
use crate::physic::solid::Solid;
use crate::player::Player;
use crate::weapon::Weapon;

pub struct ChestPlugin;

impl Plugin for ChestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_chest)
            .add_systems(Update, collect);
    }
}

#[derive(Component)]
pub struct Chest {
    weapon: Weapon,
}

#[derive(Bundle)]
pub struct ChestBundle {
    chest: Chest,
    collider: Collision,
    sprite: Sprite,
}

impl ChestBundle {
    pub fn new(
        solid: Query<&Collider, With<Solid>>,
        assets_server: &Res<AssetServer>,
        resolution: &Res<Resolution>,
    ) -> ChestBundle {
        let weapon = Weapon::random();
        let chest_texture = assets_server.load(weapon.get_asset_name());
        let collider = PolygonCollider::new(Box::new([
            Vec2::new(-15., 15.),
            Vec2::new(15., 15.),
            Vec2::new(15., -15.),
            Vec2::new(-15., -15.),
        ]));
        let position = get_random_position(Collider::Polygon(collider.clone()), solid, resolution);
        let spawn_location = Transform::from_translation(position)
            .with_scale(Vec3::splat(resolution.chest_pixel_ratio));
        ChestBundle {
            chest: Chest { weapon },
            collider: Collision::new(Collider::Polygon(collider), spawn_location),
            sprite: Sprite::from_image(chest_texture),
        }
    }
}

fn spawn_chest(
    solid: Query<&Collider, With<Solid>>,
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    resolution: Res<Resolution>,
) {
    commands.spawn(ChestBundle::new(solid, &assets_server, &resolution));
}

fn collect(
    mut commands: Commands,
    chest_query: Query<(Entity, &Chest, &Collider)>,
    mut player_query: Query<(&mut Weapon, &Collider), With<Player>>,
) {
    for (mut weapon, player_c) in player_query.iter_mut() {
        for (entity, chest, chest_c) in chest_query.iter() {
            if player_c.intersects(chest_c) {
                *weapon = chest.weapon.clone();
                commands.entity(entity).despawn();
            }
        }
    }
}
