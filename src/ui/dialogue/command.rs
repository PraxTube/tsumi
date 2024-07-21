use std::str::FromStr;

use bevy::prelude::*;

use crate::{
    npc::NpcDialogue,
    player::{chat::PlayerStoppedChat, Player, PlayerState},
    GameState,
};

pub struct DialogueCommandPlugin;

impl Plugin for DialogueCommandPlugin {
    fn build(&self, _app: &mut App) {}
}
