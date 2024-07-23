use bevy::prelude::*;
use bevy_yarnspinner::{events::DialogueCompleteEvent, prelude::*};

use crate::{aspect::Combiner, world::PlayerWentToBed, GameState};

use super::spawn::DialogueRoot;

#[derive(Component, Default)]
pub struct RunnerFlags {
    pub line: Option<LocalizedLine>,
}

fn spawn_dialogue_runner(
    mut commands: Commands,
    project: Res<YarnProject>,
    combiner: Res<Combiner>,
) {
    let mut dialogue_runner = project.create_dialogue_runner();
    dialogue_runner.start_node(&combiner.last_combined_aspect.to_string());
    commands.spawn((dialogue_runner, RunnerFlags::default()));
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
            spawn_dialogue_runner
                .run_if(in_state(GameState::Gaming).and_then(on_event::<PlayerWentToBed>())),
        )
        .add_systems(
            Update,
            (despawn_dialogue,).run_if(in_state(GameState::Gaming)),
        );
    }
}
