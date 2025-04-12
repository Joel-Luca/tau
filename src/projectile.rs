use bevy::prelude::*;

use crate::enemy;
use crate::resolution;

pub struct ProjectilePlugin; 

impl Plugin for ProjectilePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_projectile, update_interactions));
    }
}

#[derive(Component)]
pub struct Projectile{
    // direction : f32,
    pub speed : f32,
}

fn update_projectile(
    mut commands : Commands,
    mut projectile_query : Query<(Entity, &Projectile, &mut Transform)>,
    time : Res<Time>,
    resolution : Res<resolution::Resolution>,
) 
{
    for (entity, projectile, mut transform) in projectile_query.iter_mut() {
        transform.translation.y += projectile.speed * time.delta_secs();

        if transform.translation.y > resolution.screen_dimensions.y * 0.5 {
            commands.entity(entity).despawn();
        }
    }
}

const BULLET_RADIUS : f32 = 24.;
fn update_interactions(
    mut commands : Commands,
    mut enemy_query : Query<(&mut enemy::Enemy, &Transform),Without<enemy::Dead>>,
    mut projectile_query : Query<(Entity, &Transform),With<Projectile>>
) 
{
    for (mut enemy, enemy_transform) in enemy_query.iter_mut() {
        for (projectile_entity, projectile_transform) in projectile_query.iter_mut() {
            let projectile_pos = Vec2::new(
                projectile_transform.translation.x,
                projectile_transform.translation.y
            );
            let enemy_pos = Vec2::new(
                enemy_transform.translation.x,
                enemy_transform.translation.y
            );
            if Vec2::distance(enemy_pos, projectile_pos) < BULLET_RADIUS {
                enemy.dead = true;
                commands.entity(projectile_entity).despawn();
            }
        }
    }
}
