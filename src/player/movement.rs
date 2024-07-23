use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;
use bevy_yarnspinner::events::DialogueCompleteEvent;

use crate::world::PlayerWentToBed;
use crate::{GameAssets, GameState};

use super::input::PlayerInput;
use super::Player;

fn player_movement(player_input: Res<PlayerInput>, mut q_player: Query<(&mut Velocity, &Player)>) {
    let (mut velocity, player) = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    let direction = if player.can_move {
        player_input.move_direction
    } else {
        Vec2::ZERO
    };

    let speed = if player_input.running { 200.0 } else { 150.0 };
    velocity.linvel = direction * speed;
}

fn flip_sprite(player_input: Res<PlayerInput>, mut q_player: Query<(&mut Sprite, &Player)>) {
    let (mut sprite, player) = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if player_input.move_direction.x == 0.0 || !player.can_move {
        return;
    }

    sprite.flip_x = player_input.move_direction.x < 0.0;
}

fn update_animation(
    assets: Res<GameAssets>,
    player_input: Res<PlayerInput>,
    mut q_player: Query<(&mut AnimationPlayer2D, &Player)>,
) {
    let (mut animator, player) = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if !player.can_move {
        animator.pause();
        return;
    }

    let clip = if player_input.move_direction == Vec2::ZERO {
        assets.character_animations[0].clone()
    } else {
        assets.character_animations[1].clone()
    };

    animator.resume();
    animator.play(clip).repeat();
}

fn enable_player_movement(mut q_player: Query<&mut Player>) {
    let mut player = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    player.can_move = true;
}

fn disable_player_movement(mut q_player: Query<&mut Player>) {
    let mut player = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    player.can_move = false;
}

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_movement,
                flip_sprite,
                update_animation,
                enable_player_movement.run_if(on_event::<DialogueCompleteEvent>()),
                disable_player_movement.run_if(on_event::<PlayerWentToBed>()),
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
