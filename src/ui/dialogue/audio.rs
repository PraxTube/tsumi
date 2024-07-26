use std::str::FromStr;

use bevy::prelude::*;

use crate::{audio::PlaySound, npc::NpcDialogue, GameAssets, GameState};

#[derive(Event)]
pub struct PlayBlipEvent {
    dialogue: String,
}

impl PlayBlipEvent {
    pub fn new(dialogue: &str) -> Self {
        Self {
            dialogue: dialogue.to_string(),
        }
    }
}

fn character_sound(assets: &Res<GameAssets>, character: &str) -> PlaySound {
    let character = character.trim_start_matches('_');
    // Narrator, i.e. no character name on screen.
    if character.is_empty() {
        return PlaySound {
            volume: 0.0,
            ..default()
        };
    }

    match NpcDialogue::from_str(character) {
        Ok(r) => match r {
            NpcDialogue::Ami => PlaySound {
                clip: assets.ami_blip_sound.clone(),
                playback_rate: 2.5,
                rand_speed_intensity: 0.1,
                ..default()
            },
            NpcDialogue::Ima => PlaySound {
                clip: assets.ima_blip_sound.clone(),
                playback_rate: 2.3,
                rand_speed_intensity: 0.05,
                ..default()
            },
        },
        Err(_) => {
            if character == "???" {
                error!("You should never hardcode character name: '???' in dialogues!");
            }
            PlaySound {
                clip: assets.ima_blip_sound.clone(),
                ..default()
            }
        }
    }
}

fn play_blips(
    assets: Res<GameAssets>,
    mut ev_play_blip: EventReader<PlayBlipEvent>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    for ev in ev_play_blip.read() {
        ev_play_sound.send(character_sound(&assets, &ev.dialogue));
    }
}

pub struct DialogueAudioPlugin;

impl Plugin for DialogueAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (play_blips,).run_if(not(in_state(GameState::AssetLoading))),
        )
        .add_event::<PlayBlipEvent>();
    }
}
