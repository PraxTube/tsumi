use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{npc::narrator::TriggeredNarratorDialogue, GameAssets, GameState};

use super::GameAudio;

const MAIN_BGM_VOLUME: f64 = 0.15;
const ENDING_BGM_VOLUME: f64 = 0.6;
const BGM_FADE_OUT: f32 = 4.0;

#[derive(Component)]
struct Bgm {
    handle: Handle<AudioInstance>,
    volume: f64,
}

#[derive(Component, Deref, DerefMut)]
struct UnmuteTimer(Timer);

impl Default for UnmuteTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(3.0, TimerMode::Once))
    }
}

fn spawn_main_bgm(
    mut commands: Commands,
    assets: Res<GameAssets>,
    audio: Res<Audio>,
    game_audio: Res<GameAudio>,
) {
    let volume = game_audio.main_volume * MAIN_BGM_VOLUME;
    let handle = audio
        .play(assets.main_bgm.clone())
        .with_volume(volume)
        .looped()
        .handle();
    commands.spawn(Bgm {
        handle,
        volume: MAIN_BGM_VOLUME,
    });
}

fn update_bgm_volumes(
    game_audio: Res<GameAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    q_bgms: Query<&Bgm>,
) {
    for bgm in &q_bgms {
        if let Some(instance) = audio_instances.get_mut(bgm.handle.id()) {
            let volume = game_audio.main_volume * bgm.volume;
            instance.set_volume(volume, AudioTween::default());
        }
    }
}

fn despawn_bgms(
    mut commands: Commands,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    q_bgms: Query<(Entity, &Bgm)>,
) {
    for (entity, bgm) in &q_bgms {
        if let Some(instance) = audio_instances.get_mut(bgm.handle.id()) {
            instance.stop(AudioTween::linear(Duration::from_secs_f32(BGM_FADE_OUT)));
        }
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_ending_bgm(
    mut commands: Commands,
    assets: Res<GameAssets>,
    audio: Res<Audio>,
    game_audio: Res<GameAudio>,
) {
    let volume = game_audio.main_volume * ENDING_BGM_VOLUME;
    let handle = audio
        .play(assets.ending_bgm.clone())
        .with_volume(volume)
        .handle();
    commands.spawn(Bgm {
        handle,
        volume: ENDING_BGM_VOLUME,
    });
}

pub struct BgmPlugin;

impl Plugin for BgmPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_main_bgm)
            .add_systems(OnEnter(GameState::Ending), despawn_bgms)
            .add_systems(Update, update_bgm_volumes)
            .add_systems(
                Update,
                (spawn_ending_bgm.run_if(on_event::<TriggeredNarratorDialogue>()))
                    .run_if(in_state(GameState::Ending)),
            );
    }
}
