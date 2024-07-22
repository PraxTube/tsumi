mod combiner;
mod icon;
mod socket;

pub use combiner::Combiner;

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
    Anger,
    Nostalgia,
    Test,
    Blocking,
}

#[derive(Default, Component)]
pub struct AspectSocketInitiater {
    aspect: Aspect,
    on_left_side: bool,
}

impl AspectSocketInitiater {
    fn from_field(entity_instance: &EntityInstance) -> Self {
        let aspect = match entity_instance.get_enum_field("aspect") {
            Ok(r) => Aspect::from_str(r).unwrap_or_default(),
            Err(_) => Aspect::default(),
        };
        let on_left_side = match entity_instance.get_bool_field("on_left_side") {
            Ok(r) => r.to_owned(),
            Err(err) => {
                error!("counld not find field, {}", err);
                false
            }
        };
        Self {
            aspect,
            on_left_side,
        }
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
struct AspectCombiner;
