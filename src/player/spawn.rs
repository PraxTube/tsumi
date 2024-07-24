use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::world::camera::YSort;
use crate::{GameAssets, GameState};

use super::{Player, PLAYER_PIVOT};

#[derive(Component, Default)]
pub struct PlayerSpawnPos;

impl PlayerSpawnPos {
    fn from_field(_entity_instance: &EntityInstance) -> Self {
        Self
    }
}

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerSpawnPosBundle {
    #[with(PlayerSpawnPos::from_field)]
    player_spawn_pos: PlayerSpawnPos,
    #[grid_coords]
    grid_coords: GridCoords,
    #[worldly]
    worldly: Worldly,
}

fn spawn_player(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player_spawn_pos: Query<&GridCoords, Added<PlayerSpawnPos>>,
) {
    let grid_coords = match q_player_spawn_pos.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    let collider = commands
        .spawn((
            Collider::ball(12.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::default(),
            TransformBundle::from_transform(Transform::from_translation(PLAYER_PIVOT.extend(0.0))),
        ))
        .id();

    let mut animator = AnimationPlayer2D::default();
    animator
        .play(assets.character_animations[0].clone())
        .repeat();

    let pos = Vec3::new(
        grid_coords.x as f32 * 32.0,
        grid_coords.y as f32 * 32.0,
        0.0,
    );
    commands
        .spawn((
            Player {
                can_move: true,
                x_value_tutorial_dialogue: f32::MAX,
            },
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            YSort(32.0),
            animator,
            SpriteBundle {
                texture: assets.ami_texture.clone(),
                transform: Transform::from_translation(pos),
                ..default()
            },
            TextureAtlas {
                layout: assets.ami_layout.clone(),
                ..default()
            },
        ))
        .push_children(&[collider]);
}

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerSpawnPosBundle>("PlayerSpawnPos")
            .add_systems(Update, spawn_player.run_if(in_state(GameState::Gaming)));
    }
}
