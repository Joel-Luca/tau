use bevy::{
    color::palettes::css::*,
    math::bounding::*,
    prelude::*,
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
                render_shapes.run_if(in_state(ColliderState::Visible)),
                intersection_system,
                render_volumes.run_if(in_state(ColliderState::Visible)),
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
    Aabb,
    Circle,
}

#[derive(Component)]
enum Collider {
    Aabb(Aabb2d),
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

fn render_shapes(mut gizmos: Gizmos, query: Query<(&Shape, &Transform)>) {
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
            ColliderType::Aabb => {
                let mut aabb = match shape {
                    Shape::Rectangle(r) => r.aabb_2d(isometry),
                    Shape::Circle(c) => c.aabb_2d(isometry),
                    Shape::Triangle(t) => t.aabb_2d(isometry),
                    Shape::Line(l) => l.aabb_2d(isometry),
                    Shape::Capsule(c) => c.aabb_2d(isometry),
                    Shape::Polygon(p) => p.aabb_2d(isometry),
                };
                commands.entity(entity).insert(Collider::Aabb(aabb));
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

fn render_volumes(mut gizmos: Gizmos, query: Query<(&Collider, &Intersects)>) {
    for (volume, intersects) in query.iter() {
        let color = if **intersects { AQUA } else { ORANGE_RED };
        match volume {
            Collider::Aabb(a) => {
                gizmos.rect_2d(a.center(), a.half_size() * 2., color);
            }
            Collider::Circle(c) => {
                gizmos.circle_2d(c.center(), c.radius(), color);
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
                Collider::Aabb(a) => {
                    match collided_collider {
                        Collider::Aabb(collided_a) => a.intersects(collided_a),
                        Collider::Circle(collided_c) => a.intersects(collided_c),
                    }
                }
                Collider::Circle(c) => {
                    match collided_collider {
                        Collider::Aabb(collided_a) => c.intersects(collided_a),
                        Collider::Circle(collided_c) => c.intersects(collided_c),
                    }
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
