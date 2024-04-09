//! Definition of marker structs (Components), Resources and Events
use bevy::prelude::*;

// COMPONENTS

#[derive(Component, Debug)]
/// Marker struct for the Cube which has a Mesh and a Material
pub struct Cubie;

#[derive(Component, Debug)]
/// Marker struct for the 3d spaceship which is a glb model
pub struct Spaceship;

#[derive(Component, Debug)]
/// Marker struct for the Maus which is a SpriteBundle (with texture)
pub struct Mausi;

// RESOURCES

#[derive(Resource, Default)]
/// Resource for the animation state of the Spaceship
/// Used in systems: `fn click_detect_system` (see [ClickDetectorPlugin]) and `fn rotate_system_flugi`
pub struct AnimationStateResource {
    pub moving: bool,
}

// EVENTS

#[derive(Event, Default, Debug)]
/// Event used to read highscore date from the database
/// Used in `db.rs` and system `fn setup_system`
pub struct GetDataEvent;

#[derive(Event, Default, Debug)]
/// Event used to save highscore date to the database
/// Used in `db.rs`and system `fn update_highscore``
pub struct UpdateDataEvent;

// #[derive(Event, Default, Debug)]
// pub struct UpdateDataEvent {
//     pub name: String,
//     pub score: String,
// }

// PLUGINS

/// Plugin for the mouse click detection system (see [AnimationStateResource])
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
