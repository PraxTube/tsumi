pub mod runner;

mod audio;
mod spawn;
mod typewriter;
mod updating;

use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spawn::DialogueSpawnPlugin,
            updating::DialogueUpdatingPlugin,
            typewriter::DialogueTypewriterPlugin,
            runner::DialogueRunnerPlugin,
            audio::DialogueAudioPlugin,
        ));
    }
}

#[derive(Debug, Default, Clone, Copy, SystemSet, Eq, PartialEq, Hash)]
pub struct DialogueViewSystemSet;
