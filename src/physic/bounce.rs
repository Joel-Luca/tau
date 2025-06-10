use bevy::prelude::*;

use crate::physic::collision::Collider;
use crate::physic::velocity::Velocity;
use crate::wall::Wall;

pub struct BouncePlugin;

impl Plugin for BouncePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, apply_bounce);
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Bounce {
    pub bounce_count: u32,
}

fn apply_bounce(
    mut commands: Commands,
    mut bounce_query: Query<(Entity, &mut Velocity, &mut Bounce, &Collider)>,
    wall_query: Query<&Collider, With<Wall>>,
) {
    for (entity, mut velocity, mut bounce, bounce_collider) in bounce_query.iter_mut() {
        for wall_collider in wall_query.iter() {
            if bounce_collider.intersects(wall_collider) {
                if bounce.bounce_count > 0 {
                    bounce.bounce_count -= 1;
                    velocity.x = -velocity.x;
                    velocity.y = -velocity.y;
                } else {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
