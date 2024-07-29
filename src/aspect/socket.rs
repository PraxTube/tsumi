use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use bevy_rapier2d::prelude::*;

use crate::{
    player::{Player, PLAYER_PIVOT},
    world::camera::{YSort, YSortChild},
    GameAssets, GameState,
};

use super::{
    combiner::{aspect_combinations, is_socket_combination_possible, CombinedAspect, Combiner},
    icon::{icon_texture, DEFAULT_ICON_POSITION, HIGHLIGHTED_ICON_POSITION},
    name_text::AspectNameText,
    Aspect, AspectCombiner, AspectCombinerInitiater, AspectSocketInitiater,
};

const PLAYER_HIGHLIGHT_DISTANCE: f32 = 32.0;
const ASPECT_TEXT_OFFSET_TOP: Vec3 = Vec3::new(0.0, 48.0, 900.0);
const ASPECT_TEXT_OFFSET_BOTTOM: Vec3 = Vec3::new(0.0, -24.0, 900.0);
const COMBINED_ASPECT_TEXT_OFFSET: Vec3 = Vec3::new(0.0, 48.0, 900.0);
const TEXT_SCALE: Vec3 = Vec3::splat(0.1);

#[derive(Component)]
pub struct AspectIcon;
#[derive(Component)]
pub struct Socket {
    pub aspect: Aspect,
    pub on_top: bool,
}
#[derive(Component)]
pub struct CombinerIcon;
#[derive(Component)]
pub struct CombinerText;

fn spawn_bg_text(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    text: &str,
    pos: Vec3,
) -> Entity {
    let text_style = TextStyle {
        font: assets.silver_font.clone(),
        font_size: 320.0,
        color: Color::BLACK,
    };

    commands
        .spawn((Text2dBundle {
            text: Text::from_section(text, text_style).with_justify(JustifyText::Center),
            transform: Transform::from_translation(pos + Vec3::new(2.0, -2.0, -1.0))
                .with_scale(TEXT_SCALE),
            visibility: Visibility::Hidden,
            ..default()
        },))
        .id()
}

fn spawn_fg_text(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    text: &str,
    offset: Vec3,
) -> Entity {
    let text_style = TextStyle {
        font: assets.silver_font.clone(),
        font_size: 320.0,
        color: Color::WHITE,
    };

    commands
        .spawn((Text2dBundle {
            text: Text::from_section(text, text_style).with_justify(JustifyText::Center),
            transform: Transform::from_translation(offset).with_scale(TEXT_SCALE),
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
        let on_top = aspect_initiater.on_top;

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
                YSortChild(17.0),
                SpriteBundle {
                    texture: icon_texture(&assets, &aspect),
                    transform: Transform::from_translation(DEFAULT_ICON_POSITION.extend(0.0)),
                    ..default()
                },
            ))
            .id();

        let texture = if on_top {
            assets.aspect_socket_texture_left.clone()
        } else {
            assets.aspect_socket_texture_right.clone()
        };

        let aspect_string = aspect.to_string();
        let offset = if on_top {
            ASPECT_TEXT_OFFSET_TOP
        } else {
            ASPECT_TEXT_OFFSET_BOTTOM
        };
        let fg_text = spawn_fg_text(&mut commands, &assets, &aspect_string, offset);
        let bg_text = spawn_bg_text(&mut commands, &assets, &aspect_string, offset);
        commands.entity(bg_text).insert(AspectNameText);
        commands.entity(fg_text).insert(AspectNameText);

        commands
            .spawn((
                YSort(0.0),
                Socket { aspect, on_top },
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
            .push_children(&[collider, icon, fg_text, bg_text]);
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
                YSortChild(HIGHLIGHTED_ICON_POSITION.y + 1.0),
                SpriteBundle {
                    texture: icon_texture(&assets, &Aspect::NotImplemented),
                    transform: Transform::from_translation(DEFAULT_ICON_POSITION.extend(0.0))
                        .with_scale(Vec3::splat(0.0)),
                    ..default()
                },
            ))
            .id();

        let fg_text = spawn_fg_text(
            &mut commands,
            &assets,
            "SHOULD NEVER HAPPEN",
            COMBINED_ASPECT_TEXT_OFFSET,
        );
        let bg_text = spawn_bg_text(
            &mut commands,
            &assets,
            "SHOULD NEVER HAPPEN",
            COMBINED_ASPECT_TEXT_OFFSET,
        );
        commands.entity(bg_text).insert(CombinerText);
        commands.entity(fg_text).insert(CombinerText);

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
            .push_children(&[collider, icon, fg_text, bg_text]);
    }
}

fn highlight_sockets(
    combiner: Res<Combiner>,
    q_player: Query<&Transform, With<Player>>,
    mut q_sockets: Query<(&Transform, &mut TextureAtlas, &Socket), Without<Player>>,
) {
    if combiner.all_sockets_full {
        return;
    }
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
    q_sockets: Query<&Socket>,
) {
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    let (transform, mut atlas) = match q_combiner.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    let (left_aspect, right_aspect) =
        if let (Some(l_aspect), Some(r_aspect)) = (combiner.left_aspect, combiner.right_aspect) {
            (l_aspect, r_aspect)
        } else {
            atlas.index = 0;
            return;
        };

    let combined_aspect = aspect_combinations(&left_aspect, &right_aspect);
    let mut aspect_already_exists = false;
    for socket in &q_sockets {
        // The combined aspect was already combined and exists on of of the sockets
        // Prevent a second combination
        if socket.aspect == combined_aspect {
            aspect_already_exists = true;
            break;
        }
    }

    let index = if !aspect_already_exists
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

fn set_visuals_for_socket(
    assets: &Res<GameAssets>,
    combiner: &Res<Combiner>,
    q_sockets: &mut Query<(&Children, &Transform, &mut Socket), Without<Player>>,
    q_icons: &mut Query<&mut Handle<Image>, With<AspectIcon>>,
    q_texts: &mut Query<&mut Text, With<AspectNameText>>,
    on_top: bool,
) {
    if let Some((children, _, mut socket)) = q_sockets
        .iter_mut()
        .filter(|(_, _, socket)| socket.aspect == Aspect::NotImplemented && socket.on_top == on_top)
        .min_by(|(_, x_transform, _), (_, y_transform, _)| {
            x_transform
                .translation
                .x
                .total_cmp(&y_transform.translation.x)
        })
    {
        socket.aspect = combiner.last_combined_aspect;
        for child in children {
            if let Ok(mut icon) = q_icons.get_mut(*child) {
                *icon = icon_texture(assets, &combiner.last_combined_aspect);
            } else if let Ok(mut text) = q_texts.get_mut(*child) {
                text.sections[0].value = combiner.last_combined_aspect.to_string();
            }
        }
    }
}

fn push_combined_aspect(
    assets: Res<GameAssets>,
    combiner: Res<Combiner>,
    mut q_sockets: Query<(&Children, &Transform, &mut Socket), Without<Player>>,
    mut q_icons: Query<&mut Handle<Image>, With<AspectIcon>>,
    mut q_texts: Query<&mut Text, With<AspectNameText>>,
) {
    set_visuals_for_socket(
        &assets,
        &combiner,
        &mut q_sockets,
        &mut q_icons,
        &mut q_texts,
        true,
    );
    set_visuals_for_socket(
        &assets,
        &combiner,
        &mut q_sockets,
        &mut q_icons,
        &mut q_texts,
        false,
    );
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
                push_combined_aspect.run_if(on_event::<CombinedAspect>()),
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
