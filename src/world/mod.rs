pub mod camera;

mod map;

use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;
use camera::GameCameraPlugin;

use crate::{GameAssets, GameState};

pub const BACKGROUND_ZINDEX_ABS: f32 = 1_000.0;
pub const CHUNK_SIZE: f32 = 32.0 * 32.0;

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
