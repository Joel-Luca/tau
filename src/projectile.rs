use bevy::prelude::*;

use crate::collision::*;
use crate::enemy;
use crate::tank::*;

pub struct ProjectilePlugin; 

impl Plugin for ProjectilePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_for_collisions);
    }
}

#[derive(Component)]
pub struct Projectile {
    pub speed : f32,
}

const BULLET_RADIUS : f32 = 24.;
fn check_for_collisions(
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

#[derive(Component)]
pub struct Velocity{}

fn check_for_collision(
    mut commands: Commands,
    projectile_query: Query<(&mut Velocity, &Transform), With<Projectile>>,
    collider_query: Query<(Entity, &Transform, Option<&Tank>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) 
{

}

