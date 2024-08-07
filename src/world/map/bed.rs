use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;
use bevy_yarnspinner::events::DialogueCompleteEvent;

use crate::{
    aspect::{AspectCombiner, Combiner},
    player::{input::PlayerInput, Player, PLAYER_PIVOT},
    world::camera::YSort,
    GameAssets, GameState,
};

const PLAYER_HIGHLIGHT_DISTANCE: f32 = 32.0;
const COMBINER_OFFSET: Vec3 = Vec3::new(128.0, 0.0, 0.0);

#[derive(Component, Default)]
pub struct Bed;

#[derive(Event)]
pub struct PlayerWentToBed;

fn spawn_smoke_effect(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3) {
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.smoke_animations[0].clone());

    commands.spawn((
        YSort(100.0),
        animator,
        SpriteBundle {
            texture: assets.smoke_texture.clone(),
            transform: Transform::from_translation(pos).with_scale(Vec3::splat(2.0)),
            ..default()
        },
        TextureAtlas {
            layout: assets.smoke_layout.clone(),
            ..default()
        },
    ));
}

fn spawn_bed(
    mut commands: Commands,
    assets: Res<GameAssets>,
    combiner: Res<Combiner>,
    q_combiner: Query<&Transform, With<AspectCombiner>>,
) {
    if !combiner.all_sockets_full {
        return;
    }

    let combiner_transform = match q_combiner.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    let pos = combiner_transform.translation + COMBINER_OFFSET;

    spawn_smoke_effect(&mut commands, &assets, pos);

    commands.spawn((
        Bed,
        YSort(0.0),
        Collider::cuboid(16.0, 16.0),
        SpriteBundle {
            texture: assets.bed_texture.clone(),
            transform: Transform::from_translation(pos),
            ..default()
        },
        TextureAtlas {
            layout: assets.bed_layout.clone(),
            ..default()
        },
    ));
}

fn highlight_and_select_bed(
    player_input: Res<PlayerInput>,
    q_player: Query<&Transform, With<Player>>,
    mut q_bed: Query<(&Transform, &mut TextureAtlas), (With<Bed>, Without<Player>)>,
    mut ev_player_went_to_bed: EventWriter<PlayerWentToBed>,
    mut selected: Local<bool>,
) {
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    let (transform, mut atlas) = match q_bed.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    let index = if !*selected
        && transform
            .translation
            .truncate()
            .distance_squared(player_transform.translation.truncate() + PLAYER_PIVOT)
            <= PLAYER_HIGHLIGHT_DISTANCE.powi(2)
    {
        if player_input.select_socket {
            *selected = true;
            ev_player_went_to_bed.send(PlayerWentToBed);
        }
        1
    } else {
        0
    };
    atlas.index = index;
}

pub struct MapBedPlugin;

impl Plugin for MapBedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_bed.run_if(on_event::<DialogueCompleteEvent>()),
                highlight_and_select_bed,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_event::<PlayerWentToBed>();
    }
}
