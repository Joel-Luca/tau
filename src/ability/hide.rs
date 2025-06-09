use std::time::SystemTime;

use bevy::prelude::*;

pub struct HidePlugin;

#[derive(Component)]
pub struct Hide {
    pub spawn_time: SystemTime,
    pub visible_duration: u128,
}

impl Plugin for HidePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_hide);
    }
}

fn check_hide(mut commands: Commands, query: Query<(Entity, &Hide), With<(Sprite)>>) {
    for (
        entity,
        Hide {
            spawn_time,
            visible_duration,
        },
    ) in query.iter()
    {
        let duration = SystemTime::now().duration_since(*spawn_time).unwrap();
        if duration.as_millis() > *visible_duration {
            commands.entity(entity).remove::<Sprite>();
        }
    }
}
