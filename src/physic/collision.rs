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

                events.send(CollisionEvent {
                    entity,
                    collided_entity: other_entity,
                });
            }
        }
        **intersects = collied;
    }
}

fn update_text(
    mut text: Single<&mut Text>,
    cur_state: Res<State<ColliderState>>,
) {
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

pub trait BoundingVolume {}

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

        (min, max)
    }

    pub fn get_closest_vertex(&self, point: Vec2) -> Vec2 {
        let mut closest: Vec2 = Default::default();
        let mut min: f32 = f32::MAX;

        for vertex in self.vertices.iter() {
            let distance = vertex.distance(point);

            if distance < min {
                min = distance;
                closest = vertex.clone();
            }
        }

        closest
    }

    pub fn intersects_circle(&self, circle: &BoundingCircle) -> bool {
        for (index, vertex) in self.vertices.iter().enumerate() {
            let next_vertex = &self.vertices[(index + 1) % self.vertices.len()];

            let edge = next_vertex - vertex;
            let axis = Vec2::new(-edge.y, edge.x).normalize();

            let (min_a, max_a) = self.project_vertices(&self.vertices, axis);
            let (min_b, max_b) = circle.project_circle(axis);

            if min_a >= max_b || min_b >= max_a {
                return false;
            }
        }

        let closest = self.get_closest_vertex(circle.center);
        let axis = (closest - circle.center).normalize();

        let (min_a, max_a) = self.project_vertices(&self.vertices, axis);
        let (min_b, max_b) = circle.project_circle(axis);

        if min_a >= max_b || min_b >= max_a {
            return false;
        }

        true
    }
}

impl BoundingCircle {
    pub fn update_center(&mut self, transform: &Transform) {
        self.center = transform.translation.xy();
    }

    fn project_circle(&self, axis: Vec2) -> (f32, f32) {
        let direction = axis.normalize();
        let vector = self.radius * direction;
        let p1 = self.center + vector;
        let p2 = self.center - vector;

        let mut min = p1.dot(axis);
        let mut max = p2.dot(axis);

        if min > max {
            let temp = min;
            min = max;
            max = temp;
        }

        (min, max)
    }
}

pub trait IntersectsVolume<Volume: BoundingVolume + ?Sized> {
    fn intersects_volume(&self, volume: &Volume) -> bool;
}

impl IntersectsVolume<Self> for BoundingPolygon {
    fn intersects_volume(&self, other_polygon: &BoundingPolygon) -> bool {
        for (index, vertex) in self.vertices.iter().enumerate() {
            let next_vertex = &self.vertices[(index + 1) % self.vertices.len()];

            let edge = next_vertex - vertex;
            let axis = Vec2::new(-edge.y, edge.x).normalize();

            let (min_a, max_a) = self.project_vertices(&self.vertices, axis);
            let (min_b, max_b) = other_polygon.project_vertices(&other_polygon.vertices, axis);

            if min_a >= max_b || min_b >= max_a {
                return false;
            }
        }
        for (index, vertex) in other_polygon.vertices.iter().enumerate() {
            let next_vertex = &other_polygon.vertices[(index + 1) % other_polygon.vertices.len()];

            let edge = next_vertex - vertex;
            let axis = Vec2::new(-edge.y, edge.x).normalize();

            let (min_a, max_a) = self.project_vertices(&self.vertices, axis);
            let (min_b, max_b) = other_polygon.project_vertices(&other_polygon.vertices, axis);

            if min_a >= max_b || min_b >= max_a {
                return false;
            }
        }
        true
    }
}

impl IntersectsVolume<BoundingCircle> for BoundingPolygon {
    fn intersects_volume(&self, circle: &BoundingCircle) -> bool {
        self.intersects_circle(circle)
    }
}

impl IntersectsVolume<Self> for BoundingCircle {
    fn intersects_volume(&self, circle: &BoundingCircle) -> bool {
        let distance: f32 = self.center.distance(circle.center);
        let radii: f32 = self.radius + circle.radius;

        if distance >= radii {
            return false;
        }
        true
    }
}

impl IntersectsVolume<BoundingPolygon> for BoundingCircle {
    fn intersects_volume(&self, polygon: &BoundingPolygon) -> bool {
        polygon.intersects_circle(self)
    }
}
