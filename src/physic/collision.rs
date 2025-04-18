use bevy::{
    math::bounding::*,
    prelude::*,
};


pub struct CollisionPlugin; 

impl Plugin for CollisionPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_event::<CollisionEvent>()
        .add_systems(
            FixedUpdate,
            (
                check_for_circle_collision,
                check_for_box_collision,
                check_for_collisions,
                play_collision_sound,
            )
                // `chain`ing systems together runs them in order
                .chain(),
        );
    }
}

#[derive(Component)]
pub struct Collider;

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn check_for_circle_collision(circle: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
    if !circle.intersects(&bounding_box) {
        return None;
    }   

    let closest = bounding_box.closest_point(circle.center());
    let offset = circle.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}
