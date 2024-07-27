use std::time::Duration;

use bevy::prelude::*;

use crate::{audio::PlaySound, GameAssets, GameState};

use super::{input::PlayerInput, Player};

const RAND_SPEED_INTENSITY: f64 = 0.1;
const TIME_BETWEEN_STEPS_WALKING: f32 = 0.4;
const WALK_VOLUME: f64 = 0.5;

#[derive(Resource, Deref, DerefMut)]
struct StepsTimer(Timer);

impl Default for StepsTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            TIME_BETWEEN_STEPS_WALKING,
            TimerMode::Repeating,
        ))
    }
}

fn tick_steps_timers(time: Res<Time>, mut steps_timer: ResMut<StepsTimer>) {
    steps_timer.tick(time.delta());
}

fn play_step_sounds(
    assets: Res<GameAssets>,
    player_input: Res<PlayerInput>,
    mut steps_timer: ResMut<StepsTimer>,
    q_player: Query<&Player>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    let player = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    if !player.can_move || player_input.move_direction == Vec2::ZERO {
        return;
    }

    if !steps_timer.just_finished() {
        return;
    }

    steps_timer.set_duration(Duration::from_secs_f32(TIME_BETWEEN_STEPS_WALKING));
    ev_play_sound.send(PlaySound {
        clip: assets.footstep.clone(),
        volume: WALK_VOLUME,
        rand_speed_intensity: RAND_SPEED_INTENSITY,
        ..default()
    });
}

pub struct PlayerAudioPlugin;

impl Plugin for PlayerAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            ((play_step_sounds, tick_steps_timers).run_if(in_state(GameState::Gaming)),),
        )
        .init_resource::<StepsTimer>();
    }
}
