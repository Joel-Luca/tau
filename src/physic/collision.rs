use bevy::color::palettes::css::*;
use bevy::prelude::*;

use crate::configuration::controls::Controls;
use crate::physic::bounding_circle::BoundingCircle;
use crate::physic::bounding_polygon::BoundingPolygon;
use crate::physic::bounding_volume::BoundingVolume;
use crate::physic::solid::Solid;
use crate::player::Player;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ColliderState>()
            .add_event::<CollisionEvent>()
            .add_systems(Startup, setup)
            .add_systems(Update, (update_text, update_collider_state))
            .add_systems(
                PostUpdate,
                (
                    update_colliders,
                    check_player_collision,
                    render_colliders.run_if(in_state(ColliderState::Visible)),
                    intersection_system.run_if(in_state(ColliderState::Visible)),
                )
                    .chain(),
            );
    }
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum ColliderState {
    #[default]
    Hidden,
    Visible,
}

#[derive(Component)]
pub enum Collider {
    Polygon(BoundingPolygon),
    Circle(BoundingCircle),
}

impl Collider {
    pub fn intersects(&self, other: &Collider) -> bool {
        match &*self {
            Collider::Polygon(a) => match &*other {
                Collider::Polygon(collided_a) => a.intersects_volume(collided_a),
                Collider::Circle(collided_c) => a.intersects_volume(collided_c),
            },
            Collider::Circle(c) => match &*other {
                Collider::Polygon(collided_a) => c.intersects_volume(collided_a),
                Collider::Circle(collided_c) => c.intersects_volume(collided_c),
            },
        }
    }

    pub fn get_contact_vector(&self, other: &Collider) -> Vec3 {
        match &*self {
            Collider::Polygon(a) => match &*other {
                Collider::Polygon(collided_a) => a.get_contact_vector(collided_a),
                Collider::Circle(collided_c) => a.get_contact_vector(collided_c),
            },
            Collider::Circle(c) => match &*other {
                Collider::Polygon(collided_a) => c.get_contact_vector(collided_a),
                Collider::Circle(collided_c) => c.get_contact_vector(collided_c),
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct Intersects(bool);

#[derive(Event)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub collided_entity: Entity,
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn update_collider_state(
    controls: Res<Controls>,
    keycode: Res<ButtonInput<KeyCode>>,
    cur_state: Res<State<ColliderState>>,
    mut state: ResMut<NextState<ColliderState>>,
) {
    if !keycode.just_pressed(controls.debug_collider_state) {
        return;
    }

    use ColliderState::*;
    let next = match **cur_state {
        Hidden => Visible,
        Visible => Hidden,
    };
    state.set(next);
}

fn update_colliders(mut query: Query<(&mut Collider, &Transform)>) {
    for (mut collider, transform) in query.iter_mut() {
        match *collider {
            Collider::Polygon(ref mut polygon) => {
                polygon.update_vertices(transform);
            }
            Collider::Circle(ref mut circle) => {
                circle.update_center(transform);
            }
        }
    }
}

fn render_colliders(mut gizmos: Gizmos, query: Query<(&Collider, &Transform, &Intersects)>) {
    for (collider, transform, intersects) in query.iter() {
        let color = if **intersects { AQUA } else { ORANGE_RED };

        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
        let isometry = Isometry2d::new(translation, Rot2::radians(rotation));
        match collider {
            Collider::Polygon(bounding_p) => {
                let polygon: BoxedPolygon = BoxedPolygon {
                    vertices: bounding_p.relative_vertices.clone(),
                };
                gizmos.primitive_2d(&polygon, isometry, color);
            }
            Collider::Circle(bounding_c) => {
                let circle = Circle {
                    radius: bounding_c.radius,
                };
                gizmos.primitive_2d(&circle, isometry, color);
            }
        }
    }
}

fn intersection_system(
    mut collider_query: Query<(Entity, &Collider, &mut Intersects)>,
    possible_collisions: Query<(Entity, &Collider)>,
    mut events: EventWriter<CollisionEvent>,
) {
    for (entity, collider, mut intersects) in collider_query.iter_mut() {
        let mut collied: bool = false;
        for (other_entity, other_collider) in possible_collisions.iter() {
            if entity.index() == other_entity.index() {
                continue;
            }

            if collider.intersects(other_collider) {
                collied = true;

                events.write(CollisionEvent {
                    entity,
                    collided_entity: other_entity,
                });
            }
        }
        **intersects = collied;
    }
}

fn update_text(mut text: Single<&mut Text>, cur_state: Res<State<ColliderState>>) {
    if !cur_state.is_changed() {
        return;
    }

    text.clear();

    text.push_str("Intersection test:\n");
    use ColliderState::*;
    for &state in &[Hidden, Visible] {
        let s = if **cur_state == state { "*" } else { " " };
        text.push_str(&format!(" {s} {state:?} {s}\n"));
    }
    text.push_str("\nPress F1 to cycle");
}

pub trait IntersectsVolume<Volume: BoundingVolume + ?Sized> {
    fn intersects_volume(&self, volume: &Volume) -> bool;
    fn get_contact_vector(&self, volume: &Volume) -> Vec3;
}

fn check_player_collision(
    mut player_query: Query<(Entity, &mut Transform, &Player, &Collider)>,
    solid_query: Query<(Entity, &Collider), With<Solid>>,
) {
    for (player_entity, mut transform, player, player_c) in player_query.iter_mut() {
        for (solid_entity, solid_c) in solid_query.iter() {
            if player_entity.index() != solid_entity.index() && player_c.intersects(solid_c) {
                player.reset_pos(&mut transform);
                return;
            }
        }
    }
}
