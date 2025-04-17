use bevy::prelude::*;


use crate::configuration::*;
use crate::physic::collision::*;


pub struct TankPlugin; 

impl Plugin for TankPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(FixedUpdate, check_spawn_protection);
    }
}

#[derive(Component)]
pub struct Tank{
    pub deaths : i32,
    pub killable : bool,
    pub last_time_killed : f32,
    pub spawn_location : Transform, 
}

#[derive(Bundle)]
pub struct TankBundle{
    collider: Collider,
    sprite: Sprite,
    tank: Tank, 
    transform: Transform,
}

impl TankBundle {
    pub fn new(spawn_location: Transform, sprite: Sprite) -> TankBundle {
        TankBundle {
            collider: Collider{},
            sprite,
            tank: Tank { deaths: 0, killable: false, last_time_killed: 0., spawn_location:spawn_location },
            transform: spawn_location,
        }
    }
}

fn check_spawn_protection(mut tank_query : Query<&mut Tank>, time : Res<Time>, configuration : Res<Configuration>) {
    for mut tank in &mut tank_query {
        if time.delta_secs() > tank.last_time_killed + configuration.spawn_protection && tank.killable == false {
            tank.killable = true;
        }
    }
}
