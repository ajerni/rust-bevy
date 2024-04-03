use bevy::prelude::*;

#[derive(Resource)]
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
