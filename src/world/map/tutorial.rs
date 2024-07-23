use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    player::{input::PlayerInput, Player, PLAYER_PIVOT},
    world::camera::YSort,
    GameAssets, GameState,
};

const PLAYER_HIGHLIGHT_DISTANCE: f32 = 32.0;

#[derive(Default, Component)]
pub struct TutorialSwitchIntiater;

impl TutorialSwitchIntiater {
    fn from_field(_entity_instance: &EntityInstance) -> Self {
        Self
    }
}

#[derive(Default, Bundle, LdtkEntity)]
struct TutorialSwitchBundle {
    #[with(TutorialSwitchIntiater::from_field)]
    tutorial_switch_initiater: TutorialSwitchIntiater,
    #[grid_coords]
    grid_coords: GridCoords,
    #[worldly]
    worldly: Worldly,
}

#[derive(Default, Component)]
pub struct TutorialWallInitiater;

impl TutorialWallInitiater {
    fn from_field(_entity_instance: &EntityInstance) -> Self {
        Self
    }
}

#[derive(Default, Bundle, LdtkEntity)]
struct TutorialWallBundle {
    #[with(TutorialWallInitiater::from_field)]
    tutorial_wall_initiater: TutorialWallInitiater,
    #[grid_coords]
    grid_coords: GridCoords,
    #[worldly]
    worldly: Worldly,
}

#[derive(Component, Default)]
struct TutorialSwitch {
    triggerd: bool,
}

#[derive(Component)]
struct TutorialWall;

fn spawn_tutorial_switch(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_tutorial_switch: Query<&GridCoords, Added<TutorialSwitchIntiater>>,
) {
    let grid_coords = match q_tutorial_switch.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    let pos = Vec3::new(
        grid_coords.x as f32 * 32.0 + 16.0,
        grid_coords.y as f32 * 32.0 + 16.0,
        0.0,
    );

    let collider = commands
        .spawn((
            Collider::cuboid(8.0, 8.0),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))),
        ))
        .id();

    commands
        .spawn((
            TutorialSwitch::default(),
            YSort(0.0),
            SpriteBundle {
                texture: assets.tutorial_switch_texture.clone(),
                transform: Transform::from_translation(pos),
                ..default()
            },
            TextureAtlas {
                layout: assets.tutorial_switch_layout.clone(),
                ..default()
            },
        ))
        .add_child(collider);
}

fn spawn_tutorial_wall(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_tutorial_wall: Query<&GridCoords, Added<TutorialWallInitiater>>,
) {
    let grid_coords = match q_tutorial_wall.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    let pos = Vec3::new(
        grid_coords.x as f32 * 32.0 + 16.0,
        grid_coords.y as f32 * 32.0 + 16.0,
        0.0,
    );

    let collider = commands
        .spawn((
            Collider::cuboid(16.0, 16.0 * 5.0),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))),
        ))
        .id();

    commands
        .spawn((
            TutorialWall,
            YSort(100.0),
            SpriteBundle {
                texture: assets.tutorial_wall.clone(),
                transform: Transform::from_translation(pos),
                ..default()
            },
        ))
        .add_child(collider);
}

fn highlight_tutorial_switch(
    q_player: Query<&Transform, With<Player>>,
    mut q_tutorial_switch: Query<(&Transform, &mut TextureAtlas, &TutorialSwitch), Without<Player>>,
) {
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    let (transform, mut atlas, switch) = match q_tutorial_switch.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if switch.triggerd {
        return;
    }

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

fn trigger_tutorial_switch(
    mut commands: Commands,
    player_input: Res<PlayerInput>,
    mut q_tutorial_switch: Query<(&mut TextureAtlas, &mut TutorialSwitch)>,
    q_tutorial_wall: Query<Entity, With<TutorialWall>>,
) {
    if !player_input.select_socket {
        return;
    }

    let (mut atlas, mut switch) = match q_tutorial_switch.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if atlas.index == 1 {
        switch.triggerd = true;
        atlas.index = 2;
        for entity in &q_tutorial_wall {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<TutorialSwitchBundle>("TutorialSwitch")
            .register_ldtk_entity::<TutorialWallBundle>("TutorialWall")
            .add_systems(
                Update,
                (
                    spawn_tutorial_switch,
                    spawn_tutorial_wall,
                    highlight_tutorial_switch,
                    trigger_tutorial_switch,
                )
                    .run_if(in_state(GameState::Gaming)),
            );
    }
}
