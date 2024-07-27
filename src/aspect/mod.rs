mod combiner;
mod icon;
mod name_text;
mod socket;

pub use combiner::{CombinedAspect, Combiner};

use std::str::FromStr;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use strum_macros::{Display, EnumIter, EnumString};

pub struct AspectPlugin;

impl Plugin for AspectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            socket::AspectSocketPlugin,
            combiner::AspectCombinerPlugin,
            icon::AspectIconPlugin,
            name_text::AspectNameTextPlugin,
        ))
        .register_ldtk_entity::<AspectBundle>("AspectSocket")
        .register_ldtk_entity::<CombinerBundle>("CombinerSocket");
    }
}

#[derive(Default, Reflect, Clone, PartialEq, EnumString, Display, Debug, Copy, EnumIter)]
pub enum Aspect {
    #[default]
    NotImplemented,
    Joy,
    Sadness,
    Anger,
    Fear,
    Nostalgia,
    Motivation,
    Melancholy,
    Hatred,
    Vengefulness,
    Elation,
    Anticipation,
    Envy,
    Pride,
    Forgiveness,
}

#[derive(Default, Component)]
pub struct AspectSocketInitiater {
    aspect: Aspect,
    on_top: bool,
}

impl AspectSocketInitiater {
    fn from_field(entity_instance: &EntityInstance) -> Self {
        let aspect = match entity_instance.get_enum_field("aspect") {
            Ok(r) => Aspect::from_str(r).unwrap_or_default(),
            Err(_) => Aspect::default(),
        };
        let on_top = match entity_instance.get_bool_field("on_top") {
            Ok(r) => r.to_owned(),
            Err(err) => {
                error!("counld not find field, {}", err);
                false
            }
        };
        Self { aspect, on_top }
    }
}

#[derive(Component, Default)]
pub struct AspectCombinerInitiater;

impl AspectCombinerInitiater {
    fn from_field(_entity_instance: &EntityInstance) -> Self {
        Self
    }
}

#[derive(Default, Bundle, LdtkEntity)]
struct AspectBundle {
    #[with(AspectSocketInitiater::from_field)]
    aspect_initiater: AspectSocketInitiater,
    #[grid_coords]
    grid_coords: GridCoords,
    #[worldly]
    worldly: Worldly,
}

#[derive(Default, Bundle, LdtkEntity)]
struct CombinerBundle {
    #[with(AspectCombinerInitiater::from_field)]
    aspect_combiner: AspectCombinerInitiater,
    #[grid_coords]
    grid_coords: GridCoords,
    #[worldly]
    worldly: Worldly,
}

#[derive(Component, Default)]
pub struct AspectCombiner;
