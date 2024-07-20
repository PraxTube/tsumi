use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{GameAssets, GameState};

use super::input::PlayerInput;
use super::Player;

fn player_movement(
    player_input: Res<PlayerInput>,
    mut q_player: Query<&mut Velocity, With<Player>>,
) {
    let mut velocity = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    let direction = player_input.move_direction;
    let speed = if player_input.running { 200.0 } else { 150.0 };
    velocity.linvel = direction * speed;
}

fn flip_sprite(player_input: Res<PlayerInput>, mut q_player: Query<&mut Sprite, With<Player>>) {
    let mut sprite = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if player_input.move_direction.x == 0.0 {
        return;
    }

    sprite.flip_x = player_input.move_direction.x < 0.0;
}

fn update_animation(
    assets: Res<GameAssets>,
    player_input: Res<PlayerInput>,
    mut q_player: Query<(&mut AnimationPlayer2D, &Player)>,
) {
    let (mut animator, _player) = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    let clip = if player_input.move_direction == Vec2::ZERO {
        assets.player_animations[0].clone()
    } else {
        assets.player_animations[1].clone()
    };

    animator.play(clip).repeat();
}

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_movement, flip_sprite, update_animation).run_if(in_state(GameState::Gaming)),
        );
    }
}
