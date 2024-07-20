pub mod input;

mod movement;
mod spawn;

use bevy::prelude::*;

pub const PLAYER_PIVOT: Vec2 = Vec2::new(0.0, -32.0);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            input::InputPlugin,
            spawn::PlayerSpawnPlugin,
            movement::PlayerMovementPlugin,
        ));
    }
}

#[derive(Component)]
pub struct Player;
