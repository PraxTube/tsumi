mod fade_screen;
mod splash_screen;

use bevy::prelude::*;

pub struct UiScreensPlugin;

impl Plugin for UiScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            fade_screen::ScreenFadePlugin,
            splash_screen::SplashScreenPlugin,
        ));
    }
}
