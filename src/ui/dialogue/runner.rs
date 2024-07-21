use bevy::prelude::*;
use bevy_yarnspinner::{events::DialogueCompleteEvent, prelude::*};

use crate::GameState;

use super::option_selection::OptionSelection;

#[derive(Component, Default)]
pub struct RunnerFlags {
    pub line: Option<LocalizedLine>,
    pub options: Option<OptionSelection>,
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.create_dialogue_runner();
    dialogue_runner.start_node("Main");
    commands.spawn((dialogue_runner, RunnerFlags::default()));
}

fn despawn_dialogue_runner(
    mut commands: Commands,
    mut ev_dialogue_completed: EventReader<DialogueCompleteEvent>,
) {
    for ev in ev_dialogue_completed.read() {
        if let Some(r) = commands.get_entity(ev.source) {
            r.despawn_recursive();
        }
    }
}

pub struct DialogueRunnerPlugin;

impl Plugin for DialogueRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_dialogue_runner)
            .add_systems(
                Update,
                (despawn_dialogue_runner,).run_if(in_state(GameState::Gaming)),
            );
    }
}
