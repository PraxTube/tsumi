use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::TransformScaleLens, Animator, EaseFunction, Tween};

use crate::GameState;

#[derive(Component)]
struct SplashScreen;

fn spawn_splash_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("ui/bevy_icon.png");
    let image = commands
        .spawn(ImageBundle {
            image: UiImage::new(icon),
            style: Style {
                width: Val::Px(200.0),
                ..default()
            },
            ..default()
        })
        .id();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            SplashScreen,
        ))
        .add_child(image);
}

fn fade_out_splash_screen(
    mut commands: Commands,
    q_splash_screen: Query<Entity, With<SplashScreen>>,
) {
    for entity in &q_splash_screen {
        let tween = Tween::new(
            EaseFunction::ExponentialIn,
            Duration::from_secs_f32(0.5),
            TransformScaleLens {
                start: Vec3::ONE,
                end: Vec3::ZERO,
            },
        );
        commands.entity(entity).insert(Animator::new(tween));
    }
}

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::AssetLoading), spawn_splash_screen)
            .add_systems(OnExit(GameState::AssetLoading), fade_out_splash_screen);
    }
}
