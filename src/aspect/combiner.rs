use bevy::prelude::*;

use crate::player::input::PlayerInput;

use super::{socket::Socket, Aspect};

#[derive(Resource, Default, Debug)]
pub struct Combiner {
    pub left_aspect: Option<Aspect>,
    pub right_aspect: Option<Aspect>,
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

pub struct AspectCombinerPlugin;

impl Plugin for AspectCombinerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (select_aspects,))
            .init_resource::<Combiner>();
    }
}
