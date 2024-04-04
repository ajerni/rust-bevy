use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

//TESTFILE

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .run();
}
