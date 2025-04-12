use bevy::prelude::*;

use crate::projectile::*;

pub struct BulletPlugin; 

impl Plugin for BulletPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_projectile));
    }
}

#[derive(Component)]
pub struct Bullet {

}

#[derive(Bundle)]
pub struct BulletBundle {
    bullet: Bullet,
    projectile: Projectile,
}


fn update_projectile
(
    mut bullet_query : Query<(&Projectile, &mut Transform), With<Bullet>>, 
    time : Res<Time>
) 
{
    for (projectile, mut transform) in bullet_query.iter_mut() {
        let direction = transform.rotation * Vec3::Y;
        let distance = projectile.speed * time.delta_secs();
        transform.translation += direction * distance;
    }
}