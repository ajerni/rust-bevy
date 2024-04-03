use bevy::prelude::*;
//use crate::Rotator;

pub struct MyTextPlugin;

impl Plugin for MyTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, write_text_system);
    }
}

fn write_text_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        //(
        TextBundle::from_section(
            "Hello from my Text Bundle Plugin!!!",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 24.,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_self: JustifySelf::Center, // Center vertically on the screen
            //align_self: AlignSelf::Center, // Center horizontally on the screen
            top: Val::Percent(5.0),
            ..default()
        }),
        //Rotator{speed: 1.0})
    );
}
