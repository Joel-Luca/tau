use bevy::prelude::*;

use crate::physic::bounding_polygon::BoundingPolygon;
use crate::physic::collision::Collider;
use crate::physic::collision::Intersects;
use crate::physic::solid::Solid;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_walls);
    }
}

#[derive(Component)]
pub struct Wall {}

#[derive(Bundle)]
pub struct WallBundle {
    collider: Collider,
    intersects: Intersects,
    solid: Solid,
    transform: Transform,
    wall: Wall,
}

impl WallBundle {
    pub fn new(vertices: Box<[Vec2]>, transform: Transform) -> WallBundle {
        WallBundle {
            collider: Collider::Polygon(BoundingPolygon::new(vertices)),
            intersects: Intersects::default(),
            solid: Solid {},
            transform,
            wall: Wall {},
        }
    }
}

fn setup_walls(mut commands: Commands) {
    let vertices = Box::new([
        Vec2::new(0., 20.),
        Vec2::new(20., 120.),
        Vec2::new(120., 100.),
        Vec2::new(100., 0.),
    ]);
    commands.spawn(WallBundle::new(
        vertices,
        Transform::from_xyz(100., 100., 0.),
    ));
}
