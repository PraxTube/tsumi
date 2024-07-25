pub mod camera;

mod map;

pub use map::TriggerFirstDialogue;

use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;
use camera::GameCameraPlugin;

use crate::{ui::EndingTriggered, GameState};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameCameraPlugin, map::MapPlugin))
            .add_systems(OnExit(GameState::AssetLoading), configure_physics)
            .add_systems(
                Update,
                (transition_ending_state.run_if(on_event::<EndingTriggered>()))
                    .run_if(in_state(GameState::Gaming)),
            );
    }
}

fn configure_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

fn transition_ending_state(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Ending);
}
