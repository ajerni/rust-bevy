use bevy::prelude::*;
use bevy_mod_picking::events::{Click, Pointer};
use bevy_mod_picking::prelude::On;

use crate::timers::MyTimer;
use crate::schnecke::*;

pub fn make_button(
    mut commands : Commands,
    asset_server : Res<AssetServer>,
){

    commands.spawn((
        ImageBundle {
            image: asset_server.load("textures/click.png").into(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                rotation: Default::default(),
                scale: Vec3::new(0.5, 0.5, 0.5),
            },
            style: Style {
                display: Display::Flex,
                justify_self: JustifySelf::Center,
                top: Val::Percent(45.0),
                ..Default::default()
            },
            ..Default::default()
        },
        
    )).insert(On::<Pointer<Click>>::run(button_callback_click));

}

fn button_callback_click(
    mut query: Query<&mut Transform, With<UiImage>>,
    mut timer: ResMut<MyTimer>,
    asset_server: Res<AssetServer>,
    commands: Commands,
) {
    println!("Button clicked!");
    //make_beep(&mut asset_server, &mut commands);

    let mut transform = query.single_mut();
    transform.scale = Vec3::new(0.3, 0.3, 0.3);

    timer.0.reset();

    //spawn Schnecke again:
    let schnecke_pos = Vec3::new(-700.0, -300.0, 0.0);
    spawn_schnecke(commands, asset_server, schnecke_pos);
}