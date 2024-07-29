use bevy::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween};

use crate::{world::camera::YSortChild, GameAssets};

use super::{
    combiner::Combiner,
    socket::{AspectIcon, Socket},
    Aspect,
};

pub const DEFAULT_ICON_POSITION: Vec2 = Vec2::new(0.0, 16.0);
pub const HIGHLIGHTED_ICON_POSITION: Vec2 = Vec2::new(0.0, 24.0);
const DEHIGHLIGHTED_ICON_POSITION: Vec2 = Vec2::new(0.0, 8.0);
const REPOSITION_TIME: f32 = 0.2;

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
    commands: &mut Commands,
    q_icons: &Query<(Entity, &Transform), With<AspectIcon>>,
    children: &Children,
    pos: Vec2,
) {
    for child in children.iter() {
        let (entity, transform) = match q_icons.get(*child) {
            Ok(r) => r,
            Err(_) => continue,
        };

        let tween = Tween::new(
            EaseFunction::CubicOut,
            std::time::Duration::from_secs_f32(REPOSITION_TIME),
            TransformPositionLens {
                start: transform.translation,
                end: pos.extend(0.0),
            },
        );
        commands.entity(entity).insert(Animator::new(tween));
    }
}

fn set_icons_pos(
    mut commands: Commands,
    combiner: Res<Combiner>,
    q_sockets: Query<(&Children, &Socket)>,
    q_icons: Query<(Entity, &Transform), With<AspectIcon>>,
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
        set_icon_pos(&mut commands, &q_icons, children, pos);
    }
}

fn update_icon_ysorts(mut q_icons: Query<(&Transform, &mut YSortChild), With<AspectIcon>>) {
    for (transform, mut ysort) in &mut q_icons {
        *ysort = YSortChild(transform.translation.y + 1.0);
    }
}

pub struct AspectIconPlugin;

impl Plugin for AspectIconPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (set_icons_pos, update_icon_ysorts));
    }
}
