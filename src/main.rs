#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod assets;
mod audio;
mod player;
mod utils;
mod world;

pub use assets::GameAssets;
pub type GameRng = rand_xoshiro::Xoshiro256PlusPlus;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowMode, WindowResolution};

use bevy_asset_loader::prelude::*;
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::Animation2DPlugin;
use bevy_tweening::*;

const BACKGROUND_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    Gaming,
    Ending,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Fifo,
                        mode: WindowMode::Windowed,
                        fit_canvas_to_parent: false,
                        canvas: Some("#game-canvas".to_string()),
                        resolution: WindowResolution::new(1280.0, 720.0),
                        ..default()
                    }),
                    ..default()
                })
                .build(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin {
                enabled: false,
                ..default()
            },
            ParticleSystemPlugin,
            Animation2DPlugin,
            TweeningPlugin,
        ))
        .insert_resource(Msaa::Off)
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Gaming)
                .load_collection::<GameAssets>(),
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins((
            world::WorldPlugin,
            audio::GameAudioPlugin,
            player::PlayerPlugin,
            utils::UtilsPlugin,
        ))
        .run();
}
