use bevy::prelude::*;

use crate::{player::input::PlayerInput, GameAssets, GameState};

use super::{
    icon::icon_texture,
    socket::{CombinerIcon, Socket},
    Aspect,
};

#[derive(Resource, Default, Debug)]
pub struct Combiner {
    pub left_aspect: Option<Aspect>,
    pub right_aspect: Option<Aspect>,
}

fn aspect_combinations(left_aspect: &Aspect, right_aspect: &Aspect) -> Aspect {
    fn match_aspects(left_aspect: &Aspect, right_aspect: &Aspect) -> Aspect {
        match (left_aspect, right_aspect) {
            (Aspect::Nostalgia, Aspect::Anger) => Aspect::Test,
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

    let mut left_aspect = combiner.left_aspect.clone();
    let mut right_aspect = combiner.right_aspect.clone();
    for (atlas, socket) in &q_sockets {
        if atlas.index == 0 {
            continue;
        }

        if socket.on_left_side {
            left_aspect = if combiner.left_aspect != Some(socket.aspect.clone()) {
                Some(socket.aspect.clone())
            } else {
                None
            };
        } else {
            right_aspect = if combiner.right_aspect != Some(socket.aspect.clone()) {
                Some(socket.aspect.clone())
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
    combiner: Res<Combiner>,
    mut q_combiner_icon: Query<(&mut Handle<Image>, &mut Visibility), With<CombinerIcon>>,
) {
    let (mut texture, mut visibility) = match q_combiner_icon.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    let (left_aspect, right_aspect) = if let (Some(l_aspect), Some(r_aspect)) =
        (combiner.left_aspect.clone(), combiner.right_aspect.clone())
    {
        (l_aspect, r_aspect)
    } else {
        return;
    };

    *visibility = Visibility::Inherited;
    let combined_aspect = aspect_combinations(&left_aspect, &right_aspect);
    *texture = icon_texture(&assets, &combined_aspect);
}

fn hide_combiner_icon(
    combiner: Res<Combiner>,
    mut q_combiner_icon: Query<&mut Visibility, With<CombinerIcon>>,
) {
    let mut visibility = match q_combiner_icon.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if combiner.left_aspect.is_none() || combiner.right_aspect.is_none() {
        *visibility = Visibility::Hidden;
    }
}

pub struct AspectCombinerPlugin;

impl Plugin for AspectCombinerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (select_aspects, show_combiner_icon, hide_combiner_icon)
                .run_if(in_state(GameState::Gaming)),
        )
        .init_resource::<Combiner>();
    }
}
