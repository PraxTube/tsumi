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
const ASPECT_TEXT_OFFSET: Vec3 = Vec3::new(0.0, -24.0, 900.0);
const COMBINED_ASPECT_TEXT_OFFSET: Vec3 = Vec3::new(0.0, 48.0, 900.0);

#[derive(Component)]
pub struct AspectIcon;
#[derive(Component)]
pub struct Socket {
    pub aspect: Aspect,
    pub on_left_side: bool,
}
#[derive(Component)]
pub struct CombinerIcon;
#[derive(Component)]
pub struct CombinerText;

fn spawn_sub_text(commands: &mut Commands, assets: &Res<GameAssets>, text: &str) -> Entity {
    let sub_text_style = TextStyle {
        font: assets.silver_font.clone(),
        font_size: 320.0,
        color: Color::BLACK,
    };

    commands
        .spawn((Text2dBundle {
            text: Text::from_section(text, sub_text_style).with_justify(JustifyText::Center),
            transform: Transform::from_translation(Vec3::new(16.0, -16.0, -1.0)),
            ..default()
        },))
        .id()
}

fn spawn_main_text(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    text: &str,
    offset: Vec3,
) -> Entity {
    let main_text_style = TextStyle {
        font: assets.silver_font.clone(),
        font_size: 320.0,
        color: Color::WHITE,
    };

    commands
        .spawn((Text2dBundle {
            text: Text::from_section(text, main_text_style).with_justify(JustifyText::Center),
            transform: Transform::from_translation(offset).with_scale(Vec3::splat(0.1)),
            visibility: Visibility::Hidden,
            ..default()
        },))
        .id()
}

fn spawn_aspect_sockets(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_items: Query<(&AspectSocketInitiater, &GridCoords), Added<AspectSocketInitiater>>,
) {
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

        let texture = if on_left_side {
            assets.aspect_socket_texture_left.clone()
        } else {
            assets.aspect_socket_texture_right.clone()
        };

        let aspect_string = aspect.to_string();
        let text = spawn_main_text(&mut commands, &assets, &aspect_string, ASPECT_TEXT_OFFSET);
        let sub_text = spawn_sub_text(&mut commands, &assets, &aspect_string);
        commands
            .entity(text)
            .insert(AspectNameText)
            .add_child(sub_text);

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

        let text = spawn_main_text(
            &mut commands,
            &assets,
            "SHOULD NEVER HAPPEN",
            COMBINED_ASPECT_TEXT_OFFSET,
        );
        let sub_text = spawn_sub_text(&mut commands, &assets, "SHOULD NEVER HAPPEN");
        commands.entity(sub_text).insert(CombinerText);
        commands
            .entity(text)
            .insert(CombinerText)
            .add_child(sub_text);

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
            .push_children(&[collider, icon, text]);
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
        if socket.aspect == Aspect::NotImplemented {
            continue;
        }

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

    let index = if !combiner.is_blocking()
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
