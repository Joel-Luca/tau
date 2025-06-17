use bevy::prelude::*;

use crate::physic::collision::collider::Collider;
use crate::physic::solid::Solid;
use crate::physic::velocity::Velocity;

pub struct BouncePlugin;

impl Plugin for BouncePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, apply_bounce);
    }
}

#[derive(Component)]
pub struct Bounce {
    pub bounce_count: u32,
    pub last_bounce: Entity,
}

fn apply_bounce(
    mut commands: Commands,
    mut bounce_query: Query<(Entity, &mut Velocity, &mut Bounce, &Collider)>,
    solid_query: Query<(Entity, &Collider), With<Solid>>,
) {
    for (entity, mut velocity, mut bounce, bounce_collider) in bounce_query.iter_mut() {
        for (wall_entity, wall_collider) in solid_query.iter() {
            if bounce_collider.intersects(wall_collider)
                && bounce.last_bounce.index() != wall_entity.index()
            {
                if bounce.bounce_count > 0 {
                    bounce.bounce_count -= 1;
                    bounce.last_bounce = wall_entity;
                    let surface = bounce_collider.get_contact_vector(wall_collider);
                    **velocity = -velocity.reflect(surface.normalize());
                } else {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
