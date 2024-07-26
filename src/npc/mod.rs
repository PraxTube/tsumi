pub mod narrator;

use strum_macros::{Display, EnumString};

use bevy::prelude::*;

use crate::GameAssets;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(narrator::NarratorPlugin);
    }
}

#[derive(Clone, Copy, Display, PartialEq, EnumString, Default)]
pub enum NpcDialogue {
    #[default]
    Ami,
    Ima,
}

pub fn npc_character_icon(assets: &Res<GameAssets>, npc: &NpcDialogue) -> Handle<Image> {
    match npc {
        NpcDialogue::Ami => assets.ami_character_icon.clone(),
        NpcDialogue::Ima => assets.ima_character_icon.clone(),
    }
}
