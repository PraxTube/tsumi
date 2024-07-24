pub mod camera;

mod map;

pub use map::TriggerFirstDialogue;

use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;
use camera::GameCameraPlugin;

use crate::GameState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameCameraPlugin, map::MapPlugin))
            .add_systems(OnExit(GameState::AssetLoading), configure_physics);
    }
}

fn configure_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}
