use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{
    lens::{TransformPositionLens, TransformScaleLens},
    Animator, EaseFunction, Tracks, Tween,
};

use crate::{
    aspect::icon::{DEFAULT_ICON_POSITION, HIGHLIGHTED_ICON_POSITION},
    audio::PlaySound,
    player::input::PlayerInput,
    GameAssets, GameState,
};

use super::{
    icon::icon_texture,
    socket::{CombinerIcon, Socket},
    Aspect, AspectCombiner,
};

#[derive(Event)]
pub struct CombinedAspect;

#[derive(Resource, Default, Debug)]
pub struct Combiner {
    pub left_aspect: Option<Aspect>,
    pub right_aspect: Option<Aspect>,
    pub current_combination: Option<Aspect>,
    pub last_combined_aspect: Aspect,
    pub all_sockets_full: bool,
}

pub fn is_socket_combination_possible(combiner: &Res<Combiner>, socket: &Socket) -> bool {
    let combiner_aspect = if socket.on_top {
        match combiner.right_aspect {
            Some(r) => r,
            None => return true,
        }
    } else {
        match combiner.left_aspect {
            Some(r) => r,
            None => return true,
        }
    };
    aspect_combinations(&socket.aspect, &combiner_aspect) != Aspect::NotImplemented
}

pub fn aspect_combinations(left_aspect: &Aspect, right_aspect: &Aspect) -> Aspect {
    fn match_aspects(left_aspect: &Aspect, right_aspect: &Aspect) -> Aspect {
        match (left_aspect, right_aspect) {
            (Aspect::Joy, Aspect::Sadness) => Aspect::Nostalgia,
            (Aspect::Joy, Aspect::Nostalgia) => Aspect::Motivation,
            (Aspect::Sadness, Aspect::Nostalgia) => Aspect::Melancholy,
            (Aspect::Anger, Aspect::Fear) => Aspect::Hatred,
            (Aspect::Anger, Aspect::Hatred) => Aspect::Vengefulness,
            (Aspect::Joy, Aspect::Motivation) => Aspect::Elation,
            (Aspect::Hatred, Aspect::Motivation) => Aspect::Pride,
            (Aspect::Nostalgia, Aspect::Motivation) => Aspect::Anticipation,
            (Aspect::Anger, Aspect::Pride) => Aspect::Envy,
            (Aspect::Anticipation, Aspect::Elation) => Aspect::Forgiveness,
            _ => Aspect::NotImplemented,
        }
    }

    let result_aspect = match_aspects(left_aspect, right_aspect);
    if result_aspect != Aspect::NotImplemented {
        result_aspect
    } else {
        match_aspects(right_aspect, left_aspect)
    }
}

fn select_aspects(
    assets: Res<GameAssets>,
    player_input: Res<PlayerInput>,
    mut combiner: ResMut<Combiner>,
    q_sockets: Query<(&TextureAtlas, &Socket)>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    if !player_input.select_socket {
        return;
    }

    let mut left_aspect = combiner.left_aspect;
    let mut right_aspect = combiner.right_aspect;
    for (atlas, socket) in &q_sockets {
        if atlas.index == 0 {
            continue;
        }

        if socket.on_top {
            left_aspect = if combiner.left_aspect != Some(socket.aspect) {
                ev_play_sound.send(PlaySound {
                    clip: assets.select_aspect.clone(),
                    ..default()
                });
                Some(socket.aspect)
            } else {
                ev_play_sound.send(PlaySound {
                    clip: assets.deselect_aspect.clone(),
                    ..default()
                });
                None
            };
        } else {
            right_aspect = if combiner.right_aspect != Some(socket.aspect) {
                ev_play_sound.send(PlaySound {
                    clip: assets.select_aspect.clone(),
                    ..default()
                });
                Some(socket.aspect)
            } else {
                ev_play_sound.send(PlaySound {
                    clip: assets.deselect_aspect.clone(),
                    ..default()
                });
                None
            };
        }
    }

    combiner.left_aspect = left_aspect;
    combiner.right_aspect = right_aspect;
}

fn show_combiner_icon(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut combiner: ResMut<Combiner>,
    mut q_combiner_icon: Query<(Entity, &mut Handle<Image>), With<CombinerIcon>>,
    mut visible: Local<bool>,
) {
    let (entity, mut texture) = match q_combiner_icon.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if let (Some(left_aspect), Some(right_aspect)) = (combiner.left_aspect, combiner.right_aspect) {
        let combined_aspect = aspect_combinations(&left_aspect, &right_aspect);
        combiner.current_combination = Some(combined_aspect);
        *texture = icon_texture(&assets, &combined_aspect);

        if !*visible {
            *visible = true;

            let seq = Tracks::new([
                Tween::new(
                    EaseFunction::QuarticOut,
                    Duration::from_secs_f32(0.2),
                    TransformPositionLens {
                        start: DEFAULT_ICON_POSITION.extend(0.0),
                        end: HIGHLIGHTED_ICON_POSITION.extend(0.0),
                    },
                ),
                Tween::new(
                    EaseFunction::QuarticOut,
                    Duration::from_secs_f32(0.2),
                    TransformScaleLens {
                        start: Vec3::splat(0.0),
                        end: Vec3::splat(1.0),
                    },
                ),
            ]);
            commands.entity(entity).insert(Animator::new(seq));
        }
    } else if *visible {
        *visible = false;

        let seq = Tracks::new([
            Tween::new(
                EaseFunction::QuarticOut,
                Duration::from_secs_f32(0.2),
                TransformPositionLens {
                    start: HIGHLIGHTED_ICON_POSITION.extend(0.0),
                    end: DEFAULT_ICON_POSITION.extend(0.0),
                },
            ),
            Tween::new(
                EaseFunction::QuarticOut,
                Duration::from_secs_f32(0.2),
                TransformScaleLens {
                    start: Vec3::splat(1.0),
                    end: Vec3::splat(0.0),
                },
            ),
        ]);
        commands.entity(entity).insert(Animator::new(seq));
    }
}

fn select_combined_aspect(
    player_input: Res<PlayerInput>,
    mut combiner: ResMut<Combiner>,
    q_combiner: Query<&TextureAtlas, With<AspectCombiner>>,
    mut ev_combined_aspect: EventWriter<CombinedAspect>,
) {
    if !player_input.select_socket {
        return;
    }
    let atlas = match q_combiner.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    if atlas.index != 1 {
        return;
    }

    let (left_aspect, right_aspect) =
        if let (Some(l_aspect), Some(r_aspect)) = (combiner.left_aspect, combiner.right_aspect) {
            (l_aspect, r_aspect)
        } else {
            return;
        };

    let combined_aspect = aspect_combinations(&left_aspect, &right_aspect);

    combiner.last_combined_aspect = combined_aspect;
    combiner.left_aspect = None;
    combiner.right_aspect = None;
    ev_combined_aspect.send(CombinedAspect);
}

fn check_all_aspects_full(mut combiner: ResMut<Combiner>, q_sockets: Query<&Socket>) {
    combiner.all_sockets_full = q_sockets
        .iter()
        .filter(|s| s.aspect == Aspect::NotImplemented)
        .count()
        == 0;
}

pub struct AspectCombinerPlugin;

impl Plugin for AspectCombinerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                select_aspects,
                show_combiner_icon,
                select_combined_aspect,
                check_all_aspects_full,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .init_resource::<Combiner>()
        .add_event::<CombinedAspect>();
    }
}
