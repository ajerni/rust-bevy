//! Timer used for button animation
use bevy::prelude::*;

#[derive(Resource)]
/// Resource for the timer that is used in `button.rs` and `fn button_timer_system` in `main.rs`
pub struct MyTimer(pub Timer);

impl MyTimer {
    pub fn new() -> Self {
        Self(Timer::from_seconds(0.2, TimerMode::Once))
    }
}

// added to App with:  .init_resource::<MyTimer>()
impl Default for MyTimer {
    fn default() -> Self {
        Self::new()
    }
}
