use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use bevy_rapier2d::prelude::*;

use crate::{
    player::{Player, PLAYER_PIVOT},
    world::camera::{YSort, YSortChild},
    GameAssets, GameState,
};

use super::{
    combiner::{is_socket_combination_possible, Combiner},
    icon::{icon_texture, DEFAULT_ICON_POSITION},
    name_text::AspectNameText,
    Aspect, AspectCombiner, AspectCombinerInitiater, AspectSocketInitiater,
};

const PLAYER_HIGHLIGHT_DISTANCE: f32 = 32.0;

#[derive(Component)]
pub struct AspectIcon;
#[derive(Component)]
pub struct Socket {
    pub aspect: Aspect,
    pub on_left_side: bool,
}
#[derive(Component)]
pub struct CombinerIcon;

fn spawn_aspect_sockets(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_items: Query<(&AspectSocketInitiater, &GridCoords), Added<AspectSocketInitiater>>,
) {
    let text_style = TextStyle {
        font: assets.silver_font.clone(),
        font_size: 320.0,
        color: Color::WHITE,
    };
    let sub_text_style = TextStyle {
        font: assets.silver_font.clone(),
        font_size: 320.0,
        color: Color::BLACK,
    };

    for (aspect_initiater, grid_coords) in &q_items {
        let aspect = aspect_initiater.aspect;
        let on_left_side = aspect_initiater.on_left_side;

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

        let icon = commands
            .spawn((
                AspectIcon,
                YSortChild(100.0),
                SpriteBundle {
                    texture: icon_texture(&assets, &aspect),
                    transform: Transform::from_translation(DEFAULT_ICON_POSITION.extend(0.0)),
                    ..default()
                },
            ))
            .id();

        let sub_text = commands
            .spawn((Text2dBundle {
                text: Text::from_section(aspect.to_string(), sub_text_style.clone())
                    .with_justify(JustifyText::Center),
                transform: Transform::from_translation(Vec3::new(16.0, -16.0, -1.0)),
                ..default()
            },))
            .id();
        let text = commands
            .spawn((
                AspectNameText,
                Text2dBundle {
                    text: Text::from_section(aspect.to_string(), text_style.clone())
                        .with_justify(JustifyText::Center),
                    transform: Transform::from_translation(Vec3::new(0.0, -24.0, 900.0))
                        .with_scale(Vec3::splat(0.1)),
                    visibility: Visibility::Hidden,
                    ..default()
                },
            ))
            .add_child(sub_text)
            .id();

        let texture = if on_left_side {
            assets.aspect_socket_texture_left.clone()
        } else {
            assets.aspect_socket_texture_right.clone()
        };

        commands
            .spawn((
                YSort(0.0),
                Socket {
                    aspect,
                    on_left_side,
                },
                SpriteBundle {
                    texture,
                    transform: Transform::from_translation(pos),
                    ..default()
                },
                TextureAtlas {
                    layout: assets.aspect_socket_layout.clone(),
                    ..default()
                },
            ))
            .push_children(&[collider, icon, text]);
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

        let icon = commands
            .spawn((
                CombinerIcon,
                YSortChild(100.0),
                SpriteBundle {
                    texture: icon_texture(&assets, &Aspect::NotImplemented),
                    transform: Transform::from_translation(DEFAULT_ICON_POSITION.extend(0.0)),
                    visibility: Visibility::Hidden,
                    ..default()
                },
            ))
            .id();

        commands
            .spawn((
                YSort(0.0),
                AspectCombiner,
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
            .push_children(&[collider, icon]);
    }
}

fn highlight_sockets(
    combiner: Res<Combiner>,
    q_player: Query<&Transform, With<Player>>,
    mut q_sockets: Query<(&Transform, &mut TextureAtlas, &Socket), Without<Player>>,
) {
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    for (transform, mut atlas, socket) in &mut q_sockets {
        let index = if is_socket_combination_possible(&combiner, socket)
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
}

fn highlight_combiner(
    combiner: Res<Combiner>,
    q_player: Query<&Transform, With<Player>>,
    mut q_combiner: Query<(&Transform, &mut TextureAtlas), (With<AspectCombiner>, Without<Player>)>,
) {
    if !combiner.is_full() {
        return;
    }
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    let (transform, mut atlas) = match q_combiner.get_single_mut() {
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
