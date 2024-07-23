use bevy::prelude::*;

#[derive(Event)]
pub struct EndingTriggered;

pub fn trigger_ending_command(
    In(()): In<()>,
    mut ev_ending_triggered: EventWriter<EndingTriggered>,
) {
    ev_ending_triggered.send(EndingTriggered);
}

pub struct DialogueCommandPlugin;

impl Plugin for DialogueCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EndingTriggered>();
    }
}
