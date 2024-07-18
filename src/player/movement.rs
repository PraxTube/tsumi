use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GameState;

use super::input::PlayerInput;
use super::Player;

fn player_movement(
    player_input: Res<PlayerInput>,
    mut q_player: Query<&mut Velocity, With<Player>>,
) {
    let mut velocity = match q_player.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    let direction = player_input.move_direction;
    let speed = 100.0;
    velocity.linvel = direction * speed;
    info!("{}", velocity.linvel);
}

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_movement,).run_if(in_state(GameState::Gaming)),
        );
    }
}
