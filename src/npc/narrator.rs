use bevy_yarnspinner::events::DialogueCompleteEvent;
use strum_macros::{Display, EnumIter, EnumString};

use bevy::prelude::*;

use crate::{
    aspect::{Aspect, Socket},
    GameState,
};

const START_DELAY: f32 = 1.0;
const ENDING_DELAY: f32 = 2.0;
const GOOD_ENDING_THRESHOLD: i32 = 7;

#[derive(Reflect, Clone, PartialEq, EnumString, Display, Debug, Copy, EnumIter)]
pub enum NarratorDialogue {
    Intro,
    GoodEnding,
    BadEndingTooPositive,
    BadEndingTooNegative,
}

#[derive(Event)]
pub struct TriggeredNarratorDialogue(pub NarratorDialogue);

pub fn evaluate_aspect(aspect: Aspect) -> i32 {
    match aspect {
        Aspect::NotImplemented => 0,
        Aspect::Joy => 2,
        Aspect::Sadness => 0,
        Aspect::Anger => -2,
        Aspect::Fear => -1,
        Aspect::Nostalgia => 2,
        Aspect::Motivation => 3,
        Aspect::Melancholy => 2,
        Aspect::Hatred => -4,
        Aspect::Vengefulness => -6,
        Aspect::Elation => 4,
        Aspect::Anticipation => 3,
        Aspect::Envy => -5,
        Aspect::Pride => -4,
        Aspect::Forgiveness => 6,
    }
}

fn determine_ending(q_sockets: &Query<&Socket>) -> NarratorDialogue {
    let mut sum = 0;
    for socket in q_sockets {
        sum += evaluate_aspect(socket.aspect);
    }

    if sum > GOOD_ENDING_THRESHOLD {
        NarratorDialogue::BadEndingTooPositive
    } else if sum < -GOOD_ENDING_THRESHOLD {
        NarratorDialogue::BadEndingTooNegative
    } else {
        NarratorDialogue::GoodEnding
    }
}

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
    q_sockets: Query<&Socket>,
    mut ev_triggered_narrator_dialogue: EventWriter<TriggeredNarratorDialogue>,
    mut elapsed: Local<f32>,
) {
    if *elapsed > ENDING_DELAY {
        return;
    }
    *elapsed += time.delta_seconds();

    if *elapsed > ENDING_DELAY {
        ev_triggered_narrator_dialogue
            .send(TriggeredNarratorDialogue(determine_ending(&q_sockets)));
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
