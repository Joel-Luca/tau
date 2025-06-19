use bevy::color::palettes::css::*;
use bevy::prelude::*;

use crate::configuration::visibility::DebugState;
use crate::physic::collision::collider::Collider;
use crate::physic::collision::intersection::update_intersection;
use crate::physic::collision::intersection::Intersection;

pub struct VisibilityPlugin;

impl Plugin for VisibilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                update_intersection.run_if(in_state(DebugState::True)),
                render_colliders.run_if(in_state(DebugState::True)),
            )
                .chain(),
        );
    }
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
