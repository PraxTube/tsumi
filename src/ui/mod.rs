mod dialogue;
mod screens;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((dialogue::DialoguePlugin, screens::UiScreensPlugin));
    }
}
