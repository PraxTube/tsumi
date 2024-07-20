mod socket;

use std::str::FromStr;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use strum_macros::{Display, EnumString};

pub struct AspectPlugin;

impl Plugin for AspectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(socket::AspectSocketPlugin)
            .register_ldtk_entity::<AspectBundle>("AspectSocket");
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

#[derive(Resource, Default, Deref, DerefMut)]
pub struct ActiveItems(pub Vec<Aspect>);
#[derive(Resource, Deref, DerefMut)]
pub struct MaxItems(pub usize);

#[derive(Default, Bundle, LdtkEntity)]
struct AspectBundle {
    #[with(Aspect::from_field)]
    aspect: Aspect,
    #[grid_coords]
    grid_coords: GridCoords,
    #[worldly]
    worldly: Worldly,
}

impl Aspect {
    fn from_field(entity_instance: &EntityInstance) -> Aspect {
        match entity_instance.get_enum_field("aspect") {
            Ok(s) => Aspect::from_str(s).unwrap_or_default(),
            Err(_) => Aspect::default(),
        }
    }
}
