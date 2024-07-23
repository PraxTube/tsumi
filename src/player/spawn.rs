use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::world::camera::YSort;
use crate::{GameAssets, GameState};

use super::{Player, PLAYER_PIVOT};

const SPAWN_POSITION: Vec3 = Vec3::new(128.0, 128.0, 0.0);

fn spawn_player(mut commands: Commands, assets: Res<GameAssets>) {
    let collider = commands
        .spawn((
            Collider::ball(12.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::default(),
            TransformBundle::from_transform(Transform::from_translation(PLAYER_PIVOT.extend(0.0))),
        ))
        .id();

    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.player_animations[0].clone()).repeat();

    commands
        .spawn((
            Player { can_move: true },
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            YSort(32.0),
            animator,
            SpriteBundle {
                texture: assets.player_texture.clone(),
                transform: Transform::from_translation(SPAWN_POSITION),
                ..default()
            },
            TextureAtlas {
                layout: assets.player_layout.clone(),
                ..default()
            },
        ))
        .push_children(&[collider]);
}

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_player);
    }
}
