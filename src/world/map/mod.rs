use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::geometry::Collider;

use crate::{GameAssets, GameState};

const Z_LEVEL_BACKGROUND: f32 = -999.0;
const BORDER_THICKNESS: f32 = 10.0;
const ROOM_SIZE: Vec2 = Vec2::new(512.0, 512.0);
const INPADDING: Vec2 = Vec2::new(16.0, 16.0);

fn spawn_world_borders(mut commands: Commands) {
    commands.spawn((
        Collider::cuboid(BORDER_THICKNESS, ROOM_SIZE.y / 2.0),
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(
            -BORDER_THICKNESS + INPADDING.x,
            ROOM_SIZE.y / 2.0,
            0.0,
        ))),
    ));
    commands.spawn((
        Collider::cuboid(BORDER_THICKNESS, ROOM_SIZE.y / 2.0),
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(
            ROOM_SIZE.y + BORDER_THICKNESS - INPADDING.x,
            ROOM_SIZE.y / 2.0,
            0.0,
        ))),
    ));
    commands.spawn((
        Collider::cuboid(ROOM_SIZE.x / 2.0, BORDER_THICKNESS),
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(
            ROOM_SIZE.x / 2.0,
            -BORDER_THICKNESS + INPADDING.y,
            0.0,
        ))),
    ));
    commands.spawn((
        Collider::cuboid(ROOM_SIZE.x / 2.0, BORDER_THICKNESS),
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(
            ROOM_SIZE.x / 2.0,
            ROOM_SIZE.y + BORDER_THICKNESS - INPADDING.y,
            0.0,
        ))),
    ));
}

fn spawn_ldtk_world(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.level.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, Z_LEVEL_BACKGROUND)),
        ..default()
    });
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::index(0))
            .add_systems(
                OnEnter(GameState::Gaming),
                (spawn_world_borders, spawn_ldtk_world),
            );
    }
}
