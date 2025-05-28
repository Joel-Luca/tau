use bevy::prelude::*;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_controls);
    }
}

#[derive(Resource)]
pub struct Controls {
    pub movement: Movement,
    pub second_movement: Movement,
}

pub struct Movement {
    pub forward: KeyCode,
    pub backward: KeyCode,
    pub right: KeyCode,
    pub left: KeyCode,
    pub shoot: KeyCode,
}

impl Controls {
    pub fn new() -> Controls {
        Controls {
            movement: Movement {
                forward: KeyCode::KeyW,
                backward: KeyCode::KeyS,
                right: KeyCode::KeyD,
                left: KeyCode::KeyA,
                shoot: KeyCode::Space,
            },
            second_movement: Movement {
                forward: KeyCode::ArrowUp,
                backward: KeyCode::ArrowDown,
                right: KeyCode::ArrowRight,
                left: KeyCode::ArrowLeft,
                shoot: KeyCode::ShiftRight,
            },
        }
    }
}

fn setup_controls(mut commands: Commands) {
    commands.insert_resource(Controls::new())
}
