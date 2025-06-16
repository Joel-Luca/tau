use bevy::color::palettes::css::*;
use bevy::prelude::*;

use crate::configuration::controls::Controls;
use crate::physic::collision::collider::Collider;
use crate::physic::collision::intersection::update_intersection;
use crate::physic::collision::intersection::Intersection;

pub struct VisibilityPlugin;

impl Plugin for VisibilityPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ColliderState>()
            .add_systems(Startup, spawn_text)
            .add_systems(Update, (update_text, update_collider_state))
            .add_systems(
                PostUpdate,
                (
                    update_intersection.run_if(in_state(ColliderState::Visible)),
                    render_colliders.run_if(in_state(ColliderState::Visible)),
                )
                    .chain(),
            );
    }
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum ColliderState {
    #[default]
    Hidden,
    Visible,
}

fn spawn_text(mut commands: Commands) {
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
fn render_colliders(mut gizmos: Gizmos, query: Query<(&Collider, &Transform, &Intersection)>) {
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
