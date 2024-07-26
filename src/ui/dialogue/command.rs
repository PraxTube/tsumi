use bevy::prelude::*;

use crate::GameState;

pub fn trigger_ending_command(In(()): In<()>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Ending);
}

pub fn trigger_game_over_command(In(()): In<()>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::GameOver);
}
