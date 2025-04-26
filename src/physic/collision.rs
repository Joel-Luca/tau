use bevy::{
    color::palettes::css::*, math::bounding::*, prelude::*
};


pub struct CollisionPlugin; 

impl Plugin for CollisionPlugin{
    fn build(&self, app: &mut App) {
        app
        .init_state::<ColliderState>()
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            Update, 
            (
                update_text,
                update_volumes, 
                update_collider_state,
            )
        )
        .add_systems(
            PostUpdate, 
            (
                render_colliders.run_if(in_state(ColliderState::Visible)),
                intersection_system,
            ).chain()
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
pub enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Triangle(Triangle2d),
    Line(Segment2d),
    Capsule(Capsule2d),
    Polygon(RegularPolygon),
}

#[derive(Component)]
pub enum ColliderType {
    Polygon,
    Circle,
}

#[derive(Component)]
enum Collider {
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
    keycode: Res<ButtonInput<KeyCode>>,
    cur_state: Res<State<ColliderState>>,
    mut state: ResMut<NextState<ColliderState>>,
) {
    if !keycode.just_pressed(KeyCode::Space) {
        return;
    }

    use ColliderState::*;
    let next = match **cur_state {
        Hidden => Visible,
        Visible => Hidden,
    };
    state.set(next);
}

pub trait Bounding2d {
    fn bounding_polygon(&self, center: Vec2, rotation: f32) -> BoundingPolygon;
    // fn bounding_circle(&self, isometry: impl Into<Isometry2d>) -> BoundingCircle;
}

impl Bounding2d for RegularPolygon {
    fn bounding_polygon(&self, center: Vec2, rotation: f32) -> BoundingPolygon {
        let vertices: Vec<Vec2> = self.vertices(rotation).into_iter().collect();
        BoundingPolygon { vertices, center }
    }
}

fn render_colliders(mut gizmos: Gizmos, query: Query<(&Shape, &Transform)>) {
    let color = GRAY;
    for (shape, transform) in query.iter() {
        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
        let isometry = Isometry2d::new(translation, Rot2::radians(rotation));
        match shape {
            Shape::Rectangle(r) => {
                gizmos.primitive_2d(r, isometry, color);
            }
            Shape::Circle(c) => {
                gizmos.primitive_2d(c, isometry, color);
            }
            Shape::Triangle(t) => {
                gizmos.primitive_2d(t, isometry, color);
            }
            Shape::Line(l) => {
                gizmos.primitive_2d(l, isometry, color);
            }
            Shape::Capsule(c) => {
                gizmos.primitive_2d(c, isometry, color);
            }
            Shape::Polygon(p) => {
                gizmos.primitive_2d(p, isometry, color);
            }
        }
    }
}

fn update_volumes(
    mut commands: Commands,
    query: Query<
        (Entity, &ColliderType, &Shape, &Transform),
        Or<(Changed<ColliderType>, Changed<Shape>, Changed<Transform>)>,
    >,
) {
    for (entity, collider_type, shape, transform) in query.iter() {
        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
        let isometry = Isometry2d::new(translation, Rot2::radians(rotation));
        match collider_type {
            ColliderType::Polygon => {
                let mut aabb = match shape {
                    Shape::Rectangle(r) => BoundingPolygon { center: translation, rotation },
                    Shape::Circle(c) => c.aabb_2d(isometry),
                    Shape::Triangle(t) => t.aabb_2d(isometry),
                    Shape::Line(l) => l.aabb_2d(isometry),
                    Shape::Capsule(c) => c.aabb_2d(isometry),
                    Shape::Polygon(p) => p.bounding_polygon(translation, rotation),
                };
                commands.entity(entity).insert(Collider::Polygon(aabb));
            }
            ColliderType::Circle => {
                let circle = match shape {
                    Shape::Rectangle(r) => r.bounding_circle(isometry),
                    Shape::Circle(c) => c.bounding_circle(isometry),
                    Shape::Triangle(t) => t.bounding_circle(isometry),
                    Shape::Line(l) => l.bounding_circle(isometry),
                    Shape::Capsule(c) => c.bounding_circle(isometry),
                    Shape::Polygon(p) => p.bounding_circle(isometry),
                };
                commands
                    .entity(entity)
                    .insert(Collider::Circle(circle));
            }
        }
    }
}

fn intersection_system(
    mut collider_query: Query<(Entity, &Collider, &mut Intersects)>,
    possible_collisions: Query<(Entity, &Collider)>,
    mut events: EventWriter<CollisionEvent>,
) 
{
    for (entity, collider, mut intersects) in collider_query.iter_mut() {
        for (collided_entity, collided_collider) in possible_collisions.iter() {
            if entity.index() == collided_entity.index() {
                continue;
            }

            let hit: bool = match collider {
                Collider::Polygon(a) => {
                    match collided_collider {
                        Collider::Polygon(collided_a) => a.intersects(collided_a),
                        Collider::Circle(collided_c) => false,
                    }
                }
                Collider::Circle(c) => {
                    // TODO
                    false
                }
            };

            **intersects = hit;
            events.send(CollisionEvent { entity, collided_entity });
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

pub struct BoundingPolygon{
    pub vertices: Vec<Vec2>,
    pub center: Vec2,
}

pub struct BoundingCircle{
    pub radius: f32,
    pub center: Vec2,
}

impl BoundingPolygon{
    fn project_vertices(&self, vertices: &Vec<Vec2>, axis: Vec2) -> (f32, f32) {
        let mut min = f32::MIN;
        let mut max = f32::MAX;
        
        for vertex in vertices {
            let projecton = vertex.dot(axis);

            if projecton < min {
                min = projecton;
            }
    
            if projecton > max {
                max = projecton;
            }
        }
    
        return (min, max);
    }

    pub fn intersects(&self, other: &BoundingPolygon) -> bool {
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


