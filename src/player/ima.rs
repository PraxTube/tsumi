use bevy::prelude::*;
use bevy_trickfilm::prelude::*;
use bevy_yarnspinner::events::DialogueCompleteEvent;

use crate::{
    aspect::CombinedAspect,
    audio::PlaySound,
    world::{camera::YSort, TriggerFirstImaDialogue},
    GameAssets, GameState,
};

use super::Player;

#[derive(Component)]
struct Ima;

const OFFSET: Vec3 = Vec3::new(64.0, 0.0, 0.0);

fn spawn_ima(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player: Query<(&Transform, &Sprite), With<Player>>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    let (player_transform, sprite) = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    ev_play_sound.send(PlaySound {
        clip: assets.koto_hit_sound.clone(),
        volume: 0.2,
        ..default()
    });

    let sign = if sprite.flip_x { -1.0 } else { 1.0 };
    let pos = player_transform.translation + sign * OFFSET;

    let mut animator = AnimationPlayer2D::default();
    animator
        .play(assets.character_animations[0].clone())
        .repeat();

    commands.spawn((
        Ima,
        YSort(0.0),
        animator,
        SpriteBundle {
            texture: assets.ima_texture.clone(),
            transform: Transform::from_translation(pos),
            sprite: Sprite {
                flip_x: !sprite.flip_x,
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: assets.ima_layout.clone(),
            ..default()
        },
    ));
}

fn despawn_ima(mut commands: Commands, q_imas: Query<Entity, With<Ima>>) {
    for entity in &q_imas {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct ImaPlugin;

impl Plugin for ImaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_ima.run_if(
                    on_event::<CombinedAspect>().or_else(on_event::<TriggerFirstImaDialogue>()),
                ),
                despawn_ima.run_if(on_event::<DialogueCompleteEvent>()),
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
