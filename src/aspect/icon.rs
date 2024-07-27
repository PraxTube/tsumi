use bevy::prelude::*;

use crate::{world::camera::YSortChild, GameAssets};

use super::{
    combiner::Combiner,
    socket::{AspectIcon, Socket},
    Aspect,
};

pub const DEFAULT_ICON_POSITION: Vec2 = Vec2::new(0.0, 16.0);
const HIGHLIGHTED_ICON_POSITION: Vec2 = Vec2::new(0.0, 24.0);
const DEHIGHLIGHTED_ICON_POSITION: Vec2 = Vec2::new(0.0, 8.0);

pub fn icon_texture(assets: &Res<GameAssets>, aspect: &Aspect) -> Handle<Image> {
    match aspect {
        Aspect::Joy => assets.joy_icon.clone(),
        Aspect::Sadness => assets.sadness_icon.clone(),
        Aspect::Anger => assets.anger_icon.clone(),
        Aspect::Fear => assets.fear_icon.clone(),
        Aspect::Nostalgia => assets.nostalgia_icon.clone(),
        Aspect::Motivation => assets.motivation_icon.clone(),
        Aspect::Melancholy => assets.melancholy_icon.clone(),
        Aspect::Hatred => assets.hatred_icon.clone(),
        Aspect::Vengefulness => assets.vengefulness_icon.clone(),
        Aspect::Elation => assets.elation_icon.clone(),
        Aspect::Anticipation => assets.anticipation_icon.clone(),
        Aspect::Envy => assets.envy_icon.clone(),
        Aspect::Pride => assets.pride_icon.clone(),
        Aspect::Forgiveness => assets.forgiveness_icon.clone(),
        Aspect::NotImplemented => assets.transparent_icon.clone(),
    }
}

fn set_icon_pos(
    q_icons: &mut Query<(&mut Transform, &mut YSortChild), With<AspectIcon>>,
    children: &Children,
    pos: Vec2,
) {
    for child in children.iter() {
        let (mut transform, mut ysort) = match q_icons.get_mut(*child) {
            Ok(r) => r,
            Err(_) => continue,
        };

        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
        *ysort = YSortChild(pos.y + 1.0);
    }
}

fn set_icons_pos(
    combiner: Res<Combiner>,
    q_sockets: Query<(&Children, &Socket)>,
    mut q_icons: Query<(&mut Transform, &mut YSortChild), With<AspectIcon>>,
) {
    for (children, socket) in &q_sockets {
        let pos = if combiner.all_sockets_full
            || socket.on_top
                && combiner.left_aspect.is_some()
                && combiner.left_aspect != Some(socket.aspect)
            || !socket.on_top
                && combiner.right_aspect.is_some()
                && combiner.right_aspect != Some(socket.aspect)
        {
            DEHIGHLIGHTED_ICON_POSITION
        } else if socket.on_top && combiner.left_aspect == Some(socket.aspect)
            || !socket.on_top && combiner.right_aspect == Some(socket.aspect)
        {
            HIGHLIGHTED_ICON_POSITION
        } else {
            DEFAULT_ICON_POSITION
        };
        set_icon_pos(&mut q_icons, children, pos);
    }
}

pub struct AspectIconPlugin;

impl Plugin for AspectIconPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (set_icons_pos,));
    }
}
