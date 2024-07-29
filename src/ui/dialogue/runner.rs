use std::time::Duration;

use bevy::prelude::*;
use bevy_yarnspinner::{events::DialogueCompleteEvent, prelude::*};

use crate::{
    aspect::{CombinedAspect, Combiner},
    npc::narrator::TriggeredNarratorDialogue,
    world::{PlayerWentToBed, TriggerFirstImaDialogue},
    GameState,
};

use super::{
    command::{trigger_ending_command, trigger_game_over_command},
    spawn::DialogueRoot,
};

const SHORT_INTRO_TIMEOUT: f32 = 3.5;
pub const IMA_FINAL_DIALOGUE: &str = "ImaFinalDialogue";
pub const IMA_FIRST_ENCOUNTER: &str = "ImaFirstEncounter";
pub const IMA_FIRST_ENCOUNTER_SHORT: &str = "ImaFirstEncounterShort";

#[derive(Resource)]
struct TimeSinceGaming(Timer);

impl Default for TimeSinceGaming {
    fn default() -> Self {
        Self(Timer::new(
            Duration::from_secs_f32(SHORT_INTRO_TIMEOUT),
            TimerMode::Once,
        ))
    }
}

#[derive(Component, Default)]
pub struct RunnerFlags {
    pub line: Option<LocalizedLine>,
}

fn spawn_runner(commands: &mut Commands, project: &Res<YarnProject>, node: &str) {
    let mut dialogue_runner = project.create_dialogue_runner();
    dialogue_runner
        .commands_mut()
        .add_command("trigger_ending", trigger_ending_command)
        .add_command("game_over", trigger_game_over_command);
    dialogue_runner.start_node(node);
    commands.spawn((dialogue_runner, RunnerFlags::default()));
}

fn spawn_dialogue_runner(
    mut commands: Commands,
    project: Res<YarnProject>,
    combiner: Res<Combiner>,
) {
    spawn_runner(
        &mut commands,
        &project,
        &combiner.last_combined_aspect.to_string(),
    );
}

fn spawn_narrator_dialogue(
    mut commands: Commands,
    project: Res<YarnProject>,
    mut ev_triggered_narrator_dialogue: EventReader<TriggeredNarratorDialogue>,
) {
    for ev in ev_triggered_narrator_dialogue.read() {
        spawn_runner(&mut commands, &project, &ev.0.to_string());
    }
}

fn spawn_ima_first_encounter(
    mut commands: Commands,
    time_since_gaming: Res<TimeSinceGaming>,
    project: Res<YarnProject>,
) {
    let node = if time_since_gaming.0.finished() {
        IMA_FIRST_ENCOUNTER
    } else {
        IMA_FIRST_ENCOUNTER_SHORT
    };
    spawn_runner(&mut commands, &project, node);
}

fn spawn_ima_final_dialogue(mut commands: Commands, project: Res<YarnProject>) {
    spawn_runner(&mut commands, &project, IMA_FINAL_DIALOGUE);
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

fn tick_time_since_gaming(time: Res<Time>, mut time_since_gaming: ResMut<TimeSinceGaming>) {
    time_since_gaming.0.tick(time.delta());
}

pub struct DialogueRunnerPlugin;

impl Plugin for DialogueRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_dialogue_runner.run_if(on_event::<CombinedAspect>()),
                spawn_narrator_dialogue,
                spawn_ima_first_encounter.run_if(on_event::<TriggerFirstImaDialogue>()),
                spawn_ima_final_dialogue.run_if(on_event::<PlayerWentToBed>()),
                despawn_dialogue,
            )
                .run_if(not(in_state(GameState::AssetLoading))),
        )
        .add_systems(
            Update,
            tick_time_since_gaming.run_if(in_state(GameState::Gaming)),
        )
        .init_resource::<TimeSinceGaming>();
    }
}
