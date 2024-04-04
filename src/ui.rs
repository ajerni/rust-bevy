use bevy::prelude::*;

use crate::gamestate::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, main_menu.run_if(in_state(GameState::Menu)))
            .add_systems(Update, go_to_game.run_if(in_state(GameState::Menu)))
            .add_systems(OnEnter(GameState::Menu), show_menu)
            .add_systems(OnExit(GameState::Menu), hide_menu);
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

pub fn main_menu(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "MY MAIN MENU COMES HERE",
                    TextStyle {
                        font: default(),
                        font_size: 25.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style { ..default() }),
            );
        });
}

fn go_to_game(keyboard_input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>){
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        next_state.set(GameState::Playing);
    }
}

// TODO: solve this issue:
// works when starting in GameState::Menu, but crashes with following error when setting GameStage:Menu comming from GameState:Playing
// NoEntities("bevy_ecs::query::state::QueryState<&mut bevy_render::view::visibility::Visibility, bevy_ecs::query::filter::With<shapetest::ui::MainMenu>>")
