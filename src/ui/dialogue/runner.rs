use bevy::prelude::*;
use bevy_yarnspinner::{events::DialogueCompleteEvent, prelude::*};
use strum_macros::{Display, EnumIter, EnumString};

use crate::{
    aspect::{Aspect, CombinedAspect, Combiner, Socket},
    world::TriggerFirstDialogue,
    GameState,
};

use super::{command::trigger_ending_command, spawn::DialogueRoot};

#[derive(Reflect, Clone, PartialEq, EnumString, Display, Debug, Copy, EnumIter)]
pub enum Ending {
    GoodEnding,
    BadEnding,
}

#[derive(Component, Default)]
pub struct RunnerFlags {
    pub line: Option<LocalizedLine>,
}

fn spawn_runner(commands: &mut Commands, project: &Res<YarnProject>, node: &str) {
    let mut dialogue_runner = project.create_dialogue_runner();
    dialogue_runner
        .commands_mut()
        .add_command("trigger_ending", trigger_ending_command);
    dialogue_runner.start_node(node);
    commands.spawn((dialogue_runner, RunnerFlags::default()));
}

fn spawn_dialogue_runner(
    mut commands: Commands,
    project: Res<YarnProject>,
    combiner: Res<Combiner>,
    q_sockets: Query<&Socket>,
) {
    let mut is_final_ending = true;
    for socket in &q_sockets {
        if socket.aspect == Aspect::NotImplemented {
            is_final_ending = false;
            break;
        }
    }

    let node = if is_final_ending {
        Ending::GoodEnding.to_string()
    } else {
        combiner.last_combined_aspect.to_string()
    };
    spawn_runner(&mut commands, &project, &node);
}

fn spawn_first_dialogue(mut commands: Commands, project: Res<YarnProject>) {
    spawn_runner(&mut commands, &project, "Intro");
}

fn despawn_dialogue(
    mut commands: Commands,
    q_dialogue_root: Query<Entity, With<DialogueRoot>>,
    mut ev_dialogue_completed: EventReader<DialogueCompleteEvent>,
) {
    for ev in ev_dialogue_completed.read() {
        for entity in &q_dialogue_root {
            commands.entity(entity).despawn_recursive();
        }
        if let Some(r) = commands.get_entity(ev.source) {
            r.despawn_recursive();
        }
    }
}

pub struct DialogueRunnerPlugin;

impl Plugin for DialogueRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_dialogue_runner.run_if(on_event::<CombinedAspect>()),
                spawn_first_dialogue.run_if(on_event::<TriggerFirstDialogue>()),
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_systems(
            Update,
            (despawn_dialogue,).run_if(in_state(GameState::Gaming)),
        );
    }
}
