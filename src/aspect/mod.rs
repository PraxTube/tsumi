mod socket;

use std::str::FromStr;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use strum_macros::{Display, EnumString};

pub struct AspectPlugin;

impl Plugin for AspectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(socket::AspectSocketPlugin)
            .register_ldtk_entity::<AspectBundle>("AspectSocket")
            .register_ldtk_entity::<CombinerBundle>("CombinerSocket");
    }
}

#[derive(Debug, Default, Component, Reflect, Clone, PartialEq, EnumString, Display)]
pub enum Aspect {
    #[default]
    NotImplemented,
    Joy,
    Anger,
    Nostalgia,
}

impl Aspect {
    fn from_field(entity_instance: &EntityInstance) -> Self {
        match entity_instance.get_enum_field("aspect") {
            Ok(s) => Aspect::from_str(s).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }
}

#[derive(Component, Default)]
pub struct AspectCombiner {
    left_aspect: Option<Aspect>,
    right_aspect: Option<Aspect>,
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
    #[with(Aspect::from_field)]
    aspect: Aspect,
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
