use bevy_yarnspinner::events::DialogueCompleteEvent;
use strum_macros::{Display, EnumIter, EnumString};

use bevy::prelude::*;

use crate::GameState;

const START_DELAY: f32 = 1.0;
const ENDING_DELAY: f32 = 2.0;

#[derive(Reflect, Clone, PartialEq, EnumString, Display, Debug, Copy, EnumIter)]
pub enum NarratorDialogue {
    Intro,
    GoodEnding,
    BadEndingTooPositive,
    BadEndingTooNegative,
}

#[derive(Event)]
pub struct TriggeredNarratorDialogue(pub NarratorDialogue);

fn trigger_intro_dialogue(
    time: Res<Time>,
    mut ev_triggered_narrator_dialogue: EventWriter<TriggeredNarratorDialogue>,
    mut elapsed: Local<f32>,
) {
    if *elapsed > START_DELAY {
        return;
    }
    *elapsed += time.delta_seconds();

    if *elapsed > START_DELAY {
        ev_triggered_narrator_dialogue.send(TriggeredNarratorDialogue(NarratorDialogue::Intro));
    }
}

fn transition_to_gaming_state(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Gaming);
}

fn trigger_ending_dialogue(
    time: Res<Time>,
    mut ev_triggered_narrator_dialogue: EventWriter<TriggeredNarratorDialogue>,
    mut elapsed: Local<f32>,
) {
    if *elapsed > ENDING_DELAY {
        return;
    }
    *elapsed += time.delta_seconds();

    if *elapsed > ENDING_DELAY {
        ev_triggered_narrator_dialogue
            .send(TriggeredNarratorDialogue(NarratorDialogue::GoodEnding));
    }
}

pub struct NarratorPlugin;

impl Plugin for NarratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TriggeredNarratorDialogue>()
            .add_systems(
                Update,
                (
                    trigger_intro_dialogue,
                    transition_to_gaming_state.run_if(on_event::<DialogueCompleteEvent>()),
                )
                    .run_if(in_state(GameState::Intro)),
            )
            .add_systems(
                Update,
                trigger_ending_dialogue.run_if(in_state(GameState::Ending)),
            );
    }
}
