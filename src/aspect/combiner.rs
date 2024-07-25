use bevy::prelude::*;

use crate::{player::input::PlayerInput, GameAssets, GameState};

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
    player_input: Res<PlayerInput>,
    mut combiner: ResMut<Combiner>,
    q_sockets: Query<(&TextureAtlas, &Socket)>,
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
                Some(socket.aspect)
            } else {
                None
            };
        } else {
            right_aspect = if combiner.right_aspect != Some(socket.aspect) {
                Some(socket.aspect)
            } else {
                None
            };
        }
    }

    combiner.left_aspect = left_aspect;
    combiner.right_aspect = right_aspect;
}

fn show_combiner_icon(
    assets: Res<GameAssets>,
    mut combiner: ResMut<Combiner>,
    mut q_combiner_icon: Query<(&mut Handle<Image>, &mut Visibility), With<CombinerIcon>>,
) {
    let (mut texture, mut visibility) = match q_combiner_icon.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    let (left_aspect, right_aspect) =
        if let (Some(l_aspect), Some(r_aspect)) = (combiner.left_aspect, combiner.right_aspect) {
            (l_aspect, r_aspect)
        } else {
            *visibility = Visibility::Hidden;
            return;
        };

    *visibility = Visibility::Inherited;

    let combined_aspect = aspect_combinations(&left_aspect, &right_aspect);
    combiner.current_combination = Some(combined_aspect);
    *texture = icon_texture(&assets, &combined_aspect);
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

pub struct AspectCombinerPlugin;

impl Plugin for AspectCombinerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (select_aspects, show_combiner_icon, select_combined_aspect)
                .run_if(in_state(GameState::Gaming)),
        )
        .init_resource::<Combiner>()
        .add_event::<CombinedAspect>();
    }
}
