//! Plugin for the fly system of the spaceship.

use crate::controls::Spaceship;
use crate::gamestate::GameState;

use bevy::prelude::*;

/// # Basic Bevy stuff --> Transform = translate (position) & rotation & scale
pub struct FlyPlugin;

impl Plugin for FlyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fly_system.run_if(in_state(GameState::Playing)));
    }
}

fn fly_system(
    mut query: Query<&mut Transform, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += 0.1;
        };

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= 0.1;
        };

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            transform.translation.y += 0.1;
        };

        if keyboard_input.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= 0.1;
        };
    }
}
