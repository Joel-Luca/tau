use bevy::prelude::*;

use crate::configuration::controls::Controls;

pub struct VisibilityPlugin;

impl Plugin for VisibilityPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DebugState>()
            .add_systems(Startup, spawn_text)
            .add_systems(Update, (update_text, update_debug_state));
    }
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum DebugState {
    #[default]
    False,
    True,
}

fn spawn_text(mut commands: Commands) {
    commands.spawn((
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn update_text(mut text: Single<&mut Text>, cur_state: Res<State<DebugState>>) {
    if !cur_state.is_changed() {
        return;
    }
    text.clear();
    if cur_state.get() == &DebugState::False {
        return;
    }
    text.push_str("Debug");
}

fn update_debug_state(
    controls: Res<Controls>,
    keycode: Res<ButtonInput<KeyCode>>,
    cur_state: Res<State<DebugState>>,
    mut state: ResMut<NextState<DebugState>>,
) {
    if !keycode.just_pressed(controls.debug_collider_state) {
        return;
    }

    use DebugState::*;
    let next = match **cur_state {
        False => True,
        True => False,
    };
    state.set(next);
}
