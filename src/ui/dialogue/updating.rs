use std::str::FromStr;

use bevy::prelude::*;
use bevy::render::texture::TRANSPARENT_IMAGE_HANDLE;
use bevy_yarnspinner::{events::*, prelude::*};

use crate::npc::{npc_character_icon, NpcDialogue};
use crate::player::input::PlayerInput;
use crate::{GameAssets, GameState};

use super::runner::RunnerFlags;
use super::spawn::{DialogueCharacterIcon, DialogueContinueNode, DialogueNameNode};
use super::typewriter::Typewriter;
use super::DialogueViewSystemSet;

fn convert_name(name: &str) -> String {
    if name.starts_with('_') {
        return "???".to_string();
    }
    name.to_string()
}

fn present_line(
    mut typewriter: ResMut<Typewriter>,
    mut q_name_text: Query<&mut Text, With<DialogueNameNode>>,
    mut q_runner_flags: Query<&mut RunnerFlags>,
    mut ev_present_line: EventReader<PresentLineEvent>,
) {
    let mut name_text = match q_name_text.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    for ev in ev_present_line.read() {
        let name = convert_name(ev.line.character_name().unwrap_or_default());
        name_text.sections[0].value = name;
        typewriter.set_line(&ev.line);

        for mut flags in &mut q_runner_flags {
            flags.line = Some(ev.line.clone());
        }
    }
}

fn continue_dialogue(
    input: Res<PlayerInput>,
    typewriter: Res<Typewriter>,
    mut q_dialogue_runners: Query<(&mut DialogueRunner, &RunnerFlags)>,
    mut q_continue_visibility: Query<&mut Visibility, With<DialogueContinueNode>>,
) {
    if input.dialogue_continue && !typewriter.is_finished() {
        return;
    }

    if !input.dialogue_continue {
        return;
    }

    for (mut dialogue_runner, _) in &mut q_dialogue_runners {
        if !dialogue_runner.is_waiting_for_option_selection() && dialogue_runner.is_running() {
            dialogue_runner.continue_in_next_update();
            *q_continue_visibility.single_mut() = Visibility::Hidden;
        }
    }
}

fn update_dialogue_character_icon(
    assets: Res<GameAssets>,
    mut q_character_icon: Query<&mut UiImage, With<DialogueCharacterIcon>>,
    mut ev_present_line: EventReader<PresentLineEvent>,
) {
    let mut image = match q_character_icon.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    for ev in ev_present_line.read() {
        let texture = match ev.line.character_name() {
            Some(name) => npc_character_icon(
                &assets,
                &NpcDialogue::from_str(name.trim_start_matches('_')).unwrap_or_default(),
            ),
            None => TRANSPARENT_IMAGE_HANDLE,
        };

        image.texture = texture;
    }
}

pub struct DialogueUpdatingPlugin;

impl Plugin for DialogueUpdatingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                present_line.run_if(on_event::<PresentLineEvent>()),
                continue_dialogue,
                update_dialogue_character_icon.run_if(in_state(GameState::Gaming)),
            )
                .chain()
                .after(YarnSpinnerSystemSet)
                .in_set(DialogueViewSystemSet),
        );
    }
}
