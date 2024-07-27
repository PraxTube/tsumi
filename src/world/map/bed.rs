use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yarnspinner::events::DialogueCompleteEvent;

use crate::{
    aspect::{AspectCombiner, Combiner},
    player::{input::PlayerInput, Player, PLAYER_PIVOT},
    world::camera::YSort,
    GameAssets, GameState,
};

const PLAYER_HIGHLIGHT_DISTANCE: f32 = 48.0;
const COMBINER_OFFSET: Vec3 = Vec3::new(128.0, 0.0, 0.0);

#[derive(Component, Default)]
pub struct Bed;

#[derive(Event)]
pub struct PlayerWentToBed;

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

fn highlight_bed(
    q_player: Query<&Transform, With<Player>>,
    mut q_bed: Query<(&Transform, &mut TextureAtlas), (With<Bed>, Without<Player>)>,
) {
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    let (transform, mut atlas) = match q_bed.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    let index = if transform
        .translation
        .truncate()
        .distance_squared(player_transform.translation.truncate() + PLAYER_PIVOT)
        <= PLAYER_HIGHLIGHT_DISTANCE.powi(2)
    {
        1
    } else {
        0
    };
    atlas.index = index;
}

fn select_bed(
    player_input: Res<PlayerInput>,
    q_bed: Query<&TextureAtlas, With<Bed>>,
    mut ev_player_went_to_bed: EventWriter<PlayerWentToBed>,
) {
    if !player_input.select_socket {
        return;
    }
    let atlas = match q_bed.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    if atlas.index != 1 {
        return;
    }
    ev_player_went_to_bed.send(PlayerWentToBed);
}

pub struct MapBedPlugin;

impl Plugin for MapBedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_bed.run_if(on_event::<DialogueCompleteEvent>()),
                highlight_bed,
                select_bed,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_event::<PlayerWentToBed>();
    }
}
