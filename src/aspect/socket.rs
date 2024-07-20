use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use bevy_rapier2d::prelude::*;

use crate::{
    player::{Player, PLAYER_PIVOT},
    world::camera::{YSort, YSortChild},
    GameAssets, GameState,
};

use super::{Aspect, AspectCombiner, AspectCombinerInitiater};

const PLAYER_HIGHLIGHT_DISTANCE: f32 = 32.0;

#[derive(Component)]
struct Socket(Aspect);
#[derive(Component)]
struct CombinerSocket;

fn spawn_aspect_sockets(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_items: Query<(&Aspect, &GridCoords), Added<Aspect>>,
) {
    for (aspect, grid_coords) in &q_items {
        let pos = Vec3::new(
            grid_coords.x as f32 * 32.0,
            grid_coords.y as f32 * 32.0,
            0.0,
        );

        let collider = commands
            .spawn((
                Collider::cuboid(12.0, 12.0),
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                    0.0, 0.0, 0.0,
                ))),
            ))
            .id();

        let icon = match aspect {
            Aspect::Joy => assets.joy_icon.clone(),
            Aspect::Anger => assets.anger_icon.clone(),
            Aspect::Nostalgia => assets.nostalgia_icon.clone(),
            Aspect::NotImplemented => assets.joy_icon.clone(),
        };

        let icon = commands
            .spawn((
                YSortChild(100.0),
                SpriteBundle {
                    texture: icon,
                    transform: Transform::from_translation(Vec3::new(0.0, 16.0, 0.0)),
                    ..default()
                },
            ))
            .id();

        commands
            .spawn((
                YSort(0.0),
                Socket(aspect.clone()),
                SpriteBundle {
                    texture: assets.aspect_socket_texture.clone(),
                    transform: Transform::from_translation(pos),
                    ..default()
                },
                TextureAtlas {
                    layout: assets.aspect_socket_layout.clone(),
                    ..default()
                },
            ))
            .push_children(&[collider, icon]);
    }
}

fn spawn_combiner_socket(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_items: Query<&GridCoords, Added<AspectCombinerInitiater>>,
) {
    for grid_coords in &q_items {
        let pos = Vec3::new(
            grid_coords.x as f32 * 32.0,
            grid_coords.y as f32 * 32.0,
            0.0,
        );

        let collider = commands
            .spawn((
                Collider::cuboid(12.0, 12.0),
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                    0.0, 0.0, 0.0,
                ))),
            ))
            .id();

        commands
            .spawn((
                YSort(0.0),
                AspectCombiner::default(),
                SpriteBundle {
                    texture: assets.aspect_combiner_texture.clone(),
                    transform: Transform::from_translation(pos),
                    ..default()
                },
                TextureAtlas {
                    layout: assets.aspect_combiner_layout.clone(),
                    ..default()
                },
            ))
            .push_children(&[collider]);
    }
}

fn highlight_sockets(
    q_player: Query<&Transform, With<Player>>,
    mut q_sockets: Query<(&Transform, &mut TextureAtlas), (With<Socket>, Without<Player>)>,
) {
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    for (transform, mut atlas) in &mut q_sockets {
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
}

fn highlight_combiner(
    q_player: Query<&Transform, With<Player>>,
    mut q_sockets: Query<(&Transform, &mut TextureAtlas), (With<AspectCombiner>, Without<Player>)>,
) {
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    for (transform, mut atlas) in &mut q_sockets {
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
}

pub struct AspectSocketPlugin;

impl Plugin for AspectSocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_aspect_sockets,
                spawn_combiner_socket,
                highlight_sockets,
                highlight_combiner,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
