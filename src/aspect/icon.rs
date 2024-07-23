use bevy::prelude::*;

use crate::GameAssets;

use super::{
    combiner::Combiner,
    socket::{AspectIcon, Socket},
    Aspect,
};

pub const DEFAULT_ICON_POSITION: Vec2 = Vec2::new(0.0, 16.0);
const HIGHLIGHTED_ICON_POSITION: Vec2 = Vec2::new(0.0, 32.0);
const DEHIGHLIGHTED_ICON_POSITION: Vec2 = Vec2::new(0.0, 0.0);

pub fn icon_texture(assets: &Res<GameAssets>, aspect: &Aspect) -> Handle<Image> {
    match aspect {
        Aspect::Joy => assets.joy_icon.clone(),
        Aspect::Sadness => assets.sadness_icon.clone(),
        Aspect::Anger => assets.anger_icon.clone(),
        Aspect::Fear => assets.fear_icon.clone(),
        Aspect::Nostalgia => assets.nostalgia_icon.clone(),
        Aspect::Motivation => assets.motivation_icon.clone(),
        Aspect::Melanchony => assets.melanchony_icon.clone(),
        Aspect::Hatred => assets.hatred_icon.clone(),
        Aspect::Vengefulness => assets.vengfulness_icon.clone(),
        Aspect::Elation => assets.elation_icon.clone(),
        Aspect::NotImplemented => assets.transparent_icon.clone(),
    }
}

fn set_icon_pos(
    q_icons: &mut Query<&mut Transform, With<AspectIcon>>,
    children: &Children,
    pos: Vec2,
) {
    for child in children.iter() {
        let mut transform = match q_icons.get_mut(*child) {
            Ok(r) => r,
            Err(_) => continue,
        };

        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}

fn highlight_icons(
    combiner: Res<Combiner>,
    q_sockets: Query<(&Children, &Socket)>,
    mut q_icons: Query<&mut Transform, With<AspectIcon>>,
) {
    for (children, socket) in &q_sockets {
        if socket.on_top && combiner.left_aspect == Some(socket.aspect)
            || !socket.on_top && combiner.right_aspect == Some(socket.aspect)
        {
            set_icon_pos(&mut q_icons, children, HIGHLIGHTED_ICON_POSITION);
        }
    }
}

fn dehighlight_icons(
    combiner: Res<Combiner>,
    q_sockets: Query<(&Children, &Socket)>,
    mut q_icons: Query<&mut Transform, With<AspectIcon>>,
) {
    for (children, socket) in &q_sockets {
        if socket.on_top
            && combiner.left_aspect.is_some()
            && combiner.left_aspect != Some(socket.aspect)
            || !socket.on_top
                && combiner.right_aspect.is_some()
                && combiner.right_aspect != Some(socket.aspect)
        {
            set_icon_pos(&mut q_icons, children, DEHIGHLIGHTED_ICON_POSITION);
        }
    }
}

fn default_icons(
    combiner: Res<Combiner>,
    q_sockets: Query<(&Children, &Socket)>,
    mut q_icons: Query<&mut Transform, With<AspectIcon>>,
) {
    for (children, socket) in &q_sockets {
        if socket.on_top && combiner.left_aspect.is_none()
            || !socket.on_top && combiner.right_aspect.is_none()
        {
            set_icon_pos(&mut q_icons, children, DEFAULT_ICON_POSITION);
        }
    }
}

pub struct AspectIconPlugin;

impl Plugin for AspectIconPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (highlight_icons, dehighlight_icons, default_icons));
    }
}
