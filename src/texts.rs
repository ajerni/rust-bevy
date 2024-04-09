//! Plugin for explanation text (also containing a system to change color on pause state)
use bevy::prelude::*;

use crate::GameState;

/// Marker struct for text
pub struct MyTextPlugin;

#[derive(Component)]
struct InsturctionText;

impl Plugin for MyTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, write_text_system)
            //.add_systems(Startup, write_text_system.run_if(on_event::<MyLastLoadEvent>()))
            .add_systems(Update, change_text_color_on_paused);
    }
}

fn write_text_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "Space = Pause / ESC = Menu",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 24.,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_self: JustifySelf::End, // Center vertically on the screen
            //align_self: AlignSelf::Center, // Center horizontally on the screen
            top: Val::Percent(3.0),
            margin: UiRect {
                right: Val::Percent(2.),
                ..default()
            },
            ..default()
        }),
        InsturctionText,
    ));
}

/// checks active state and changes color of text accordingly
fn change_text_color_on_paused(
    state: Res<State<GameState>>,
    mut query: Query<&mut Text, With<InsturctionText>>,
) {
    for mut text in query.iter_mut() {
        // checks active state
        if state.get() == &GameState::Paused {
            text.sections[0].style.color = Color::RED;
        } else {
            text.sections[0].style.color = Color::WHITE;
        }
    }
}
