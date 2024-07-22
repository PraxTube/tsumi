use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    aspect::{Aspect, Combiner},
    player::{input::PlayerInput, Player, PLAYER_PIVOT},
    world::camera::YSort,
    GameAssets, GameState,
};

const PLAYER_HIGHLIGHT_DISTANCE: f32 = 48.0;

#[derive(Component, Default)]
pub struct Bed;

#[derive(Event)]
pub struct PlayerWentToBed;

impl Bed {
    fn from_field(_entity_instance: &EntityInstance) -> Self {
        Self
    }
}

#[derive(Default, Bundle, LdtkEntity)]
struct BedBundle {
    #[with(Bed::from_field)]
    bed: Bed,
    #[grid_coords]
    grid_coords: GridCoords,
    #[worldly]
    worldly: Worldly,
}

fn spawn_bed(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_items: Query<&GridCoords, Added<Bed>>,
) {
    for grid_coords in &q_items {
        let pos = Vec3::new(
            grid_coords.x as f32 * 32.0,
            grid_coords.y as f32 * 32.0,
            0.0,
        );

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
}

fn highlight_bed(
    combiner: Res<Combiner>,
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

    let is_blocking = combiner.left_aspect == Some(Aspect::Blocking)
        && combiner.right_aspect == Some(Aspect::Blocking);

    let index = if is_blocking
        && transform
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
    mut combiner: ResMut<Combiner>,
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

    combiner.left_aspect = None;
    combiner.right_aspect = None;
    ev_player_went_to_bed.send(PlayerWentToBed);
}

pub struct MapBedPlugin;

impl Plugin for MapBedPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<BedBundle>("Bed")
            .add_event::<PlayerWentToBed>()
            .add_systems(
                Update,
                (spawn_bed, highlight_bed, select_bed).run_if(in_state(GameState::Gaming)),
            );
    }
}
