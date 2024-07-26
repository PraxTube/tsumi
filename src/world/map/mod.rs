mod collisions;
mod keyboard_hint;
mod tutorial;

pub use tutorial::TriggerFirstImaDialogue;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{GameAssets, GameState};

const Z_LEVEL_BACKGROUND: f32 = -999.0;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .add_plugins((
                collisions::CollisionsPlugin,
                tutorial::TutorialPlugin,
                keyboard_hint::KeyboardHintPlugin,
            ))
            .insert_resource(LevelSelection::index(0))
            .add_systems(OnEnter(GameState::Gaming), spawn_ldtk_world);
    }
}

fn spawn_ldtk_world(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.level.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, Z_LEVEL_BACKGROUND)),
        ..default()
    });
}
