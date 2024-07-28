mod dialogue;
mod ending_text;
mod screens;
mod vignette;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            dialogue::DialoguePlugin,
            screens::UiScreensPlugin,
            ending_text::EndingTextPlugin,
            vignette::VignettePlugin,
        ));
    }
}
