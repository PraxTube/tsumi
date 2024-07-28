use bevy::prelude::*;

use crate::{
    aspect::CombinedAspect,
    world::{PlayerWentToBed, TriggerFirstImaDialogue},
    GameAssets, GameState,
};

fn spawn_vignette(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(ImageBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        image: UiImage {
            texture: assets.vignette.clone(),
            color: Color::BLACK,
            ..default()
        },
        ..default()
    });
}

pub struct VignettePlugin;

impl Plugin for VignettePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_vignette.run_if(on_event::<CombinedAspect>().or_else(
                on_event::<TriggerFirstImaDialogue>().or_else(on_event::<PlayerWentToBed>()),
            )),)
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
