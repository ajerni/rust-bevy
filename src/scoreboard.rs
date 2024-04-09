//! Displays the score and highscore
use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct Scoreboard {
    pub score: usize,
    pub highscore: usize,
    pub player_name: String,
    pub highscore_holder: String,
}

#[derive(Component, Debug)]
/// Marker struct for Scoereboard
pub struct ScoreboardUi;

#[derive(Component, Debug)]
/// Marker struct for Highscore
pub struct HighscoreUi;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(10.0);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

/// Displays the score
pub fn make_scoreboard(mut commands: Commands) {
    commands.spawn((
        ScoreboardUi,
        TextBundle::from_sections([
            TextSection::new(
                "Score (reset with 's'): ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
    ));
}

/// Displays the highscore
pub fn make_highscore(mut commands: Commands) {
    commands.spawn((
        HighscoreUi,
        TextBundle::from_sections([
            TextSection::new(
                "High-Score (reset with 'h'): ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
            TextSection::new(
                " from: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
    ));
}
