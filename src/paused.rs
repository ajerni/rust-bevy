//! Pause plugin.

use bevy::prelude::*;

use crate::gamestate::GameState;

/// Plugin for the pause state.
pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, game_state_input_events);
    }
}

/// Changing the game state based on keyboard input.
/// uses pattern matching over the game states.
fn game_state_input_events(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            _ => (),
        }
    }
}
