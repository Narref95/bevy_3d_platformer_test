use bevy::{prelude::*, diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}};

use crate::player::Player;

pub struct DebugModePlugin;

impl Plugin for DebugModePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(infotext_system)
        .add_system(update_fps_text)
        .add_system(update_player_text);
    }
}

#[derive(Component)]
struct FPSText;

#[derive(Component)]
struct PlayerInfoText;

fn infotext_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts\\FiraSans-Bold.ttf");
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Player Info",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::GREEN,
                },
            ),
            TextSection::new(
                "\nIs jumping:",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "false",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::RED,
                },
            ),
            TextSection::new(
                "\nIs grounded:",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "false",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::RED,
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(25.0),
                right: Val::Px(35.0),
                ..default()
            },
            ..default()
        }),
        PlayerInfoText,
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            )
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        FPSText,
    ));
}

fn update_fps_text(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FPSText>>,
) {
    for mut text in &mut query {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                fps = fps_smoothed;
            }
        }

        let mut frame_time = time.delta_seconds_f64();
        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_smoothed) = frame_time_diagnostic.smoothed() {
                frame_time = frame_time_smoothed;
            }
        }

        text.sections[0].value = format!(
            "{:.1} fps, {:.3} ms/frame",
            fps, frame_time,
        );
    }
}

fn update_player_text(
    mut text_query: Query<&mut Text, With<PlayerInfoText>>,
    player_query: Query<&Player, With<Player>>,
) {
    for mut text in &mut text_query {
        if let Ok(player) = player_query.get_single() {
            text.sections[2].value = player.is_jumping.to_string();
            text.sections[2].style.color = if player.is_jumping { Color::GREEN } else { Color::RED };
            text.sections[4].value = player.is_grounded.to_string();
            text.sections[4].style.color = if player.is_grounded { Color::GREEN } else { Color::RED };
        }
    }
}