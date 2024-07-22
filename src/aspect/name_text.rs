use bevy::prelude::*;

use crate::GameState;

use super::{
    socket::{CombinerText, Socket},
    AspectCombiner, Combiner,
};

#[derive(Component)]
pub struct AspectNameText;

fn show_aspect_name_texts(
    q_sockets: Query<(&Children, &TextureAtlas), With<Socket>>,
    mut q_aspect_name_texts: Query<&mut Visibility, With<AspectNameText>>,
) {
    for (children, atlas) in &q_sockets {
        if atlas.index != 1 {
            continue;
        }

        for child in children.iter() {
            let mut visibility = match q_aspect_name_texts.get_mut(*child) {
                Ok(r) => r,
                Err(_) => continue,
            };

            *visibility = Visibility::Inherited;
        }
    }
}

fn hide_aspect_name_texts(
    q_sockets: Query<(&Children, &TextureAtlas), With<Socket>>,
    mut q_aspect_name_texts: Query<&mut Visibility, With<AspectNameText>>,
) {
    for (children, atlas) in &q_sockets {
        if atlas.index != 0 {
            continue;
        }

        for child in children.iter() {
            let mut visibility = match q_aspect_name_texts.get_mut(*child) {
                Ok(r) => r,
                Err(_) => continue,
            };

            *visibility = Visibility::Hidden;
        }
    }
}

fn show_combined_aspect_name_texts(
    combiner: Res<Combiner>,
    q_combiner: Query<&TextureAtlas, With<AspectCombiner>>,
    mut q_combiner_text: Query<(&mut Visibility, &mut Text), With<CombinerText>>,
) {
    let atlas = match q_combiner.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    for (mut visibility, mut text) in &mut q_combiner_text {
        if atlas.index == 1 {
            *visibility = Visibility::Inherited;
            text.sections[0].value = combiner.current_combination.unwrap_or_default().to_string();
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

fn hide_combined_aspect_name_texts(
    q_sockets: Query<(&Children, &TextureAtlas), With<Socket>>,
    mut q_aspect_name_texts: Query<&mut Visibility, With<AspectNameText>>,
) {
    for (children, atlas) in &q_sockets {
        if atlas.index != 0 {
            continue;
        }

        for child in children.iter() {
            let mut visibility = match q_aspect_name_texts.get_mut(*child) {
                Ok(r) => r,
                Err(_) => continue,
            };

            *visibility = Visibility::Hidden;
        }
    }
}

pub struct AspectNameTextPlugin;

impl Plugin for AspectNameTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                show_aspect_name_texts,
                hide_aspect_name_texts,
                show_combined_aspect_name_texts,
                hide_combined_aspect_name_texts,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
