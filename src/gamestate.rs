//! States used to stear the games flow (which systems to run on which states)
use bevy::prelude::*;

//starting in Playing here to get initial camera

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
/// The current state of the game.
/// # See further docs inside the code on how to use these States
/// This is used to control which systems are executed.
pub enum GameState {
    #[default]
    Playing,
    Menu,
    Paused,
}

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
/// This State is used to trigger if Emitter is emitting or not
pub enum SchneckenEmitterState {
    #[default]
    NotEmitting,
    Emitting,
}

// RUN CONDITION BASED ON STATE:
// play_system.run_if(in_state(GameState::Playing)),
// pause_system.run_if(in_state(GameState::Paused)),

// SPECIAL SCHEDULES FOR STATE CHANGES:
// app.add_systems(OnEnter(MyAppState::MainMenu), (
//     setup_main_menu_ui,
// ));
// app.add_systems(OnExit(MyAppState::MainMenu), (
//     despawn_main_menu,
// ));

// CHANCHING TO NEXT STATE:
// fn toggle_pause_game(
//     state: Res<State<MyPausedState>>,
//     mut next_state: ResMut<NextState<MyPausedState>>,
// ) {
//     match state.get() {
//         MyPausedState::Paused => next_state.set(MyPausedState::Running),
//         MyPausedState::Running => next_state.set(MyPausedState::Paused),
//     }
// }

// if you have multiple states that must be set correctly,
// don't forget to manage them all
// fn new_game_multiplayer(
//     mut next_app: ResMut<NextState<MyAppState>>,
//     mut next_mode: ResMut<NextState<MyGameModeState>>,
// ) {
//     next_app.set(MyAppState::InGame);
//     next_mode.set(MyGameModeState::Multiplayer);
// }
