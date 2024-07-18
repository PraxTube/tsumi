pub mod camera;

use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;
use camera::GameCameraPlugin;

use crate::{GameAssets, GameState};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameCameraPlugin,))
            .add_systems(OnExit(GameState::AssetLoading), configure_physics)
            .add_systems(OnExit(GameState::AssetLoading), (spawn_dummy_background,));
    }
}

fn configure_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

fn spawn_dummy_background(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(SpriteBundle {
        texture: assets.dummy_background.clone(),
        ..default()
    });
}
