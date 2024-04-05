use bevy::prelude::*;
use bevy_simple_text_input::{TextInputBundle, TextInputPlugin, TextInputSubmitEvent};

use crate::gamestate::GameState;
use crate::gamestate::SchneckenEmitterState;
use crate::scoreboard::Scoreboard;

const BORDER_COLOR_ACTIVE: Color = Color::rgb(0.75, 0.52, 0.99);
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreStartup, main_menu)
            .add_systems(OnEnter(GameState::Menu), show_menu)
            .add_systems(OnExit(GameState::Menu), hide_menu)
            .add_systems(Update, escape_to_main_menu)
            .add_systems(Update, go_to_game)
            .add_systems(Update, go_to_emit)
            .add_plugins(TextInputPlugin)
            .add_systems(Update, text_input_listener);
    }
}

#[derive(Component)]
struct MainMenu;

fn show_menu(mut menu: Query<&mut Visibility, With<MainMenu>>) {
    let mut menu = menu.single_mut();
    *menu = Visibility::Visible;
}

fn hide_menu(mut menu: Query<&mut Visibility, With<MainMenu>>) {
    let mut menu = menu.single_mut();
    *menu = Visibility::Hidden;
}

pub fn main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>

) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.),
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::DARK_GREEN),
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_sections([
                    TextSection::new(
                        "M A I N   M E N U".to_string(),
                        TextStyle {
                            font: asset_server.load("fonts/Roboto-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::MIDNIGHT_BLUE,
                        },
                    ),
                ])
                .with_style(Style {
                    justify_content: JustifyContent::Center, // Center horizontally
                    position_type: PositionType::Absolute, // Use absolute positioning
                    top: Val::Px(40.0), // Add a small top margin
                    ..default()
                }),
            );
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Enter name and press Enter to start the game: \n\n\nPress W to preview the winning state",
                    TextStyle {
                        font: default(),
                        font_size: 25.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style { ..default() }),
            );
        })
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(250.0),
                        border: UiRect::all(Val::Px(5.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        top: Val::Px(-36.0),
                        ..default()
                    },
                    border_color: BORDER_COLOR_ACTIVE.into(),
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                },
                TextInputBundle::default().with_text_style(TextStyle {
                    font_size: 40.,
                    color: TEXT_COLOR,
                    ..default()
                })
                // .with_value("Click Me")
                // .with_inactive(true),
            ));
        });
}

fn go_to_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Playing);
    }
}

fn go_to_emit(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<SchneckenEmitterState>>,
    mut query: Query<&mut BackgroundColor, With<MainMenu>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyW) {
        let mut color = query.single_mut();
        color.0 = Color::Rgba {
            red: 0.0,
            green: 0.9,
            blue: 0.0,
            alpha: 0.8, //opacity
        };
        //color.0 = Color::NONE; // Set background color to transparent
        next_state.set(SchneckenEmitterState::Emitting);
    }
}

fn escape_to_main_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<&mut BackgroundColor, With<MainMenu>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
        let mut color = query.single_mut();
        color.0 = Color::DARK_GREEN;
    }
}

fn text_input_listener(
    mut events: EventReader<TextInputSubmitEvent>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    for event in events.read() {
        info!("{:?} submitted: {:?}", event.entity, event.value);
        scoreboard.player_name = event.value.clone();
    }
}
