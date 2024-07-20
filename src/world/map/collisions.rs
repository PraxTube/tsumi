use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Default)]
pub struct CollisionBox {
    width: f32,
    height: f32,
}

impl CollisionBox {
    fn from_field(entity_instance: &EntityInstance) -> Self {
        Self {
            width: entity_instance.width as f32 / 2.0,
            height: entity_instance.height as f32 / 2.0,
        }
    }
}

#[derive(Default, Bundle, LdtkEntity)]
struct CollisionBoxBundle {
    #[with(CollisionBox::from_field)]
    collision_box: CollisionBox,
    #[grid_coords]
    grid_coords: GridCoords,
    #[worldly]
    worldly: Worldly,
}

fn spawn_collision_boxes(
    mut commands: Commands,
    q_items: Query<(&GridCoords, &CollisionBox), Added<CollisionBox>>,
) {
    for (grid_coords, collision_box) in &q_items {
        let pos = Vec3::new(
            grid_coords.x as f32 * 32.0 + collision_box.width,
            grid_coords.y as f32 * 32.0 + 32.0 - collision_box.height,
            0.0,
        );

        commands.spawn((
            Collider::cuboid(collision_box.width, collision_box.height),
            TransformBundle::from_transform(Transform::from_translation(pos)),
        ));
    }
}

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<CollisionBoxBundle>("CollisionBox")
            .add_systems(Update, (spawn_collision_boxes,));
    }
}
