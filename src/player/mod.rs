pub mod input;

mod ima;
mod movement;
mod spawn;

pub use spawn::PlayerSpawnPos;

use bevy::prelude::*;

pub const PLAYER_PIVOT: Vec2 = Vec2::new(0.0, -32.0);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            input::InputPlugin,
            spawn::PlayerSpawnPlugin,
            movement::PlayerMovementPlugin,
            ima::ImaPlugin,
        ));
    }
}

#[derive(Component)]
pub struct Player {
    pub can_move: bool,
}
