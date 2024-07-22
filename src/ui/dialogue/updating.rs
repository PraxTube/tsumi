use bevy::prelude::*;
use bevy_yarnspinner::{events::*, prelude::*};

use crate::player::input::PlayerInput;

use super::runner::RunnerFlags;
use super::spawn::{DialogueContinueNode, DialogueNameNode};
use super::typewriter::{Typewriter, WriteDialogueText};
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

    for event in ev_present_line.read() {
        let name = convert_name(event.line.character_name().unwrap_or_default());
        name_text.sections[0].value = name;
        typewriter.set_line(&event.line);

        for mut flags in &mut q_runner_flags {
            flags.line = Some(event.line.clone());
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

fn update_dialogue_name(
    typewriter: Res<Typewriter>,
    mut q_name_text: Query<&mut Text, With<DialogueNameNode>>,
    mut ev_write_dialogue_text: EventReader<WriteDialogueText>,
) {
    if ev_write_dialogue_text.is_empty() {
        return;
    }
    ev_write_dialogue_text.clear();

    let mut text = match q_name_text.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    text.sections[0].value = convert_name(&typewriter.character_name.clone().unwrap_or_default());
}

pub struct DialogueUpdatingPlugin;

impl Plugin for DialogueUpdatingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                present_line.run_if(on_event::<PresentLineEvent>()),
                continue_dialogue,
                update_dialogue_name,
            )
                .chain()
                .after(YarnSpinnerSystemSet)
                .in_set(DialogueViewSystemSet),
        );
    }
}
