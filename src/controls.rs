use bevy::prelude::*;

// COMPONENTS

#[derive(Component, Debug)]
pub struct Cubie;

// #[derive(Component, Debug)]
// pub struct Schnecke;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct Mausi;

// RESOURCES

#[derive(Resource, Default)]
pub struct AnimationStateResource {
    pub moving: bool,
}

// EVENTS

#[derive(Event, Default, Debug)]
pub struct GetDataEvent;

// PLUGINS

pub struct ClickDetectorPlugin;

impl Plugin for ClickDetectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, click_detect_system);
        //app.add_systems(Update, (click_detect_system, get_rid_of_mouse));
    }
}

fn click_detect_system(
    input: Res<ButtonInput<MouseButton>>,
    mut animation_state: ResMut<AnimationStateResource>,
) {
    if input.just_pressed(MouseButton::Left) {
        //toggle moving to start/stop Spaceships rotation
        animation_state.moving = !animation_state.moving;
    };

    //println!("animation state: {:#?}", animation_state.moving);
}
