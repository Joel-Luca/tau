use crate::configuration::controls::Controls;
use bevy::{color::palettes::css::*, prelude::*};

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
                    render_colliders.run_if(in_state(ColliderState::Visible)),
                    intersection_system,
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

#[derive(Component, Deref, DerefMut, Default)]
pub struct Intersects(bool);

#[derive(Event)]
pub struct CollisionEvent {
    entity: Entity,
    collided_entity: Entity,
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
            Collider::Circle(ref mut circle) => continue,
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
        for (other_entity, other_collider) in possible_collisions.iter() {
            if entity.index() == other_entity.index() {
                continue;
            }

            let hit: bool = match &*collider {
                Collider::Polygon(a) => match &*other_collider {
                    Collider::Polygon(collided_a) => a.intersects(collided_a),
                    Collider::Circle(collided_c) => false,
                },
                Collider::Circle(c) => {
                    // TODO
                    false
                }
            };

            **intersects = hit;
            events.send(CollisionEvent {
                entity,
                collided_entity: other_entity,
            });
        }
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
    text.push_str("\nPress space to cycle");
}

pub trait BoundingVolume {}

pub trait IntersectsVolume<Volume: BoundingVolume + ?Sized> {
    fn intersects(&self, volume: &Volume) -> bool;
}

pub struct BoundingPolygon {
    pub relative_vertices: Box<[Vec2]>,
    pub vertices: Box<[Vec2]>,
}

impl BoundingVolume for BoundingPolygon {}

pub struct BoundingCircle {
    pub radius: f32,
    pub center: Vec2,
}

impl BoundingVolume for BoundingCircle {}

impl BoundingPolygon {
    pub fn new(vertices: Box<[Vec2]>) -> BoundingPolygon {
        let absolute_vertices = vertices.clone();
        BoundingPolygon {
            relative_vertices: vertices,
            vertices: absolute_vertices,
        }
    }

    pub fn update_vertices(&mut self, transform: &Transform) {
        for (index, vertex) in self.relative_vertices.iter().enumerate() {
            self.vertices[index].x = transform.translation.x + vertex.x;
            self.vertices[index].y = transform.translation.y + vertex.y;
        }
    }

    fn project_vertices(&self, vertices: &Box<[Vec2]>, axis: Vec2) -> (f32, f32) {
        let mut min = f32::MAX;
        let mut max = f32::MIN;

        for vertex in vertices {
            let projection = vertex.dot(axis);

            if projection < min {
                min = projection;
            }

            if projection > max {
                max = projection;
            }
        }

        return (min, max);
    }
}

impl IntersectsVolume<Self> for BoundingPolygon {
    fn intersects(&self, other: &BoundingPolygon) -> bool {
        for (index, vertex) in self.vertices.iter().enumerate() {
            let next_vertex = &self.vertices[(index + 1) % self.vertices.len()];

            let edge = next_vertex - vertex;
            let axis = Vec2::new(-edge.y, edge.x);

            let (min_a, max_a) = self.project_vertices(&self.vertices, axis);
            let (min_b, max_b) = other.project_vertices(&other.vertices, axis);

            if min_a >= max_b || min_b >= max_a {
                return false;
            }
        }
        for (index, vertex) in other.vertices.iter().enumerate() {
            let next_vertex = &other.vertices[(index + 1) % other.vertices.len()];

            let edge = next_vertex - vertex;
            let axis = Vec2::new(-edge.y, edge.x);

            let (min_a, max_a) = self.project_vertices(&self.vertices, axis);
            let (min_b, max_b) = other.project_vertices(&other.vertices, axis);

            if min_a >= max_b || min_b >= max_a {
                return false;
            }
        }
        return true;
    }
}
