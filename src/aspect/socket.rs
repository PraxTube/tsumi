use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use bevy_rapier2d::prelude::*;

use crate::{world::camera::YSort, GameAssets, GameState};

use super::Aspect;

#[derive(Component)]
struct Socket(Aspect);

fn spawn_aspect_sockets(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_items: Query<(&Aspect, &GridCoords), Added<Aspect>>,
) {
    for (aspect, grid_coords) in &q_items {
        info!("spawnign socket");
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
                Socket(aspect.clone()),
                SpriteBundle {
                    texture: assets.aspect_socket.clone(),
                    transform: Transform::from_translation(pos),
                    ..default()
                },
            ))
            .push_children(&[collider]);
    }
}

pub struct AspectSocketPlugin;

impl Plugin for AspectSocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_aspect_sockets,).run_if(in_state(GameState::Gaming)),
        );
    }
}
