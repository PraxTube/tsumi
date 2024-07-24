use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{player::PlayerSpawnPos, world::camera::YSort, GameAssets, GameState};

use super::tutorial::TutorialSwitchIntiater;

const SPAWN_OFFSET: Vec3 = Vec3::new(-160.0, 0.0, 0.0);
const SELECTION_KEY_SPAWN_OFFSET: Vec3 = Vec3::new(16.0, 128.0, 0.0);
const BUTTON_DIS: f32 = 40.0;
const ARROW_DIS: f32 = 80.0;
const SHIFT_DIS: f32 = 60.0;
const ICON_SIZE: f32 = 0.75;

#[derive(Component)]
struct KeyboardIcon;
#[derive(Component)]
pub struct KeyboardHint;

enum Icon {
    DownKey,
    UpKey,
    LeftKey,
    RightKey,
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
    ShiftKey,
}

fn icon_to_texture(
    assets: &Res<GameAssets>,
    icon: &Icon,
) -> (Handle<Image>, Handle<TextureAtlasLayout>) {
    match icon {
        Icon::DownKey | Icon::DownArrow => (
            assets.ui_down_key_texture.clone(),
            assets.ui_down_key_layout.clone(),
        ),
        Icon::UpKey | Icon::UpArrow => (
            assets.ui_up_key_texture.clone(),
            assets.ui_up_key_layout.clone(),
        ),
        Icon::LeftKey | Icon::LeftArrow => (
            assets.ui_left_key_texture.clone(),
            assets.ui_left_key_layout.clone(),
        ),
        Icon::RightKey | Icon::RightArrow => (
            assets.ui_right_key_texture.clone(),
            assets.ui_right_key_layout.clone(),
        ),
        Icon::ShiftKey => (
            assets.ui_shift_key_texture.clone(),
            assets.ui_shift_key_layout.clone(),
        ),
    }
}

fn spawn_icon(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    root: Entity,
    icon: Icon,
    offset: Vec2,
) -> Entity {
    let (texture, layout) = icon_to_texture(assets, &icon);
    let transform = Transform::from_translation(offset.extend(0.0));

    let icon = commands
        .spawn((
            KeyboardIcon,
            SpriteBundle {
                texture,
                transform,
                ..default()
            },
            TextureAtlas {
                layout,
                ..default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[icon]);
    icon
}

fn spawn_animated_icon(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    root: Entity,
    icon: Icon,
    offset: Vec2,
) {
    let entity = spawn_icon(commands, assets, root, icon, offset);

    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.ui_keys_animations[0].clone()).repeat();

    commands
        .entity(entity)
        .insert((Collider::cuboid(16.0, 16.0), animator));
}

fn spawn_unanimated_icon(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    root: Entity,
    icon: Icon,
    offset: Vec2,
) {
    let entity = spawn_icon(commands, assets, root, icon, offset);

    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.ui_keys_animations[1].clone()).repeat();

    commands.entity(entity).insert(animator);
}

fn spawn_shift_icon(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    root: Entity,
    icon: Icon,
    offset: Vec2,
) {
    let entity = spawn_icon(commands, assets, root, icon, offset);

    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.ui_keys_animations[2].clone()).repeat();

    commands
        .entity(entity)
        .insert((Collider::cuboid(32.0, 16.0), animator));
}

fn spawn_keyboard_ui(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player_position: Query<&GridCoords, Added<PlayerSpawnPos>>,
) {
    let grid_coords = match q_player_position.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    let pos = Vec3::new(
        grid_coords.x as f32 * 32.0,
        grid_coords.y as f32 * 32.0,
        0.0,
    );

    let transform =
        Transform::from_translation(pos + SPAWN_OFFSET).with_scale(Vec3::splat(ICON_SIZE));
    let root = commands
        .spawn((
            KeyboardHint,
            YSort(-200.0),
            SpatialBundle::from_transform(transform),
        ))
        .id();

    spawn_animated_icon(
        &mut commands,
        &assets,
        root,
        Icon::DownKey,
        Vec2::new(0.0, -BUTTON_DIS),
    );
    spawn_unanimated_icon(
        &mut commands,
        &assets,
        root,
        Icon::DownArrow,
        Vec2::new(0.0, -ARROW_DIS),
    );
    spawn_animated_icon(
        &mut commands,
        &assets,
        root,
        Icon::UpKey,
        Vec2::new(0.0, BUTTON_DIS),
    );
    spawn_unanimated_icon(
        &mut commands,
        &assets,
        root,
        Icon::UpArrow,
        Vec2::new(0.0, ARROW_DIS),
    );
    spawn_animated_icon(
        &mut commands,
        &assets,
        root,
        Icon::LeftKey,
        Vec2::new(-BUTTON_DIS, 0.0),
    );
    spawn_unanimated_icon(
        &mut commands,
        &assets,
        root,
        Icon::LeftArrow,
        Vec2::new(-ARROW_DIS, 0.0),
    );
    spawn_animated_icon(
        &mut commands,
        &assets,
        root,
        Icon::RightKey,
        Vec2::new(BUTTON_DIS, 0.0),
    );
    spawn_unanimated_icon(
        &mut commands,
        &assets,
        root,
        Icon::RightArrow,
        Vec2::new(ARROW_DIS, 0.0),
    );
    spawn_shift_icon(
        &mut commands,
        &assets,
        root,
        Icon::ShiftKey,
        Vec2::new(SHIFT_DIS, SHIFT_DIS),
    );
}

fn spawn_selection_key_ui(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_tutorial_switch: Query<&GridCoords, Added<TutorialSwitchIntiater>>,
) {
    let grid_coords = match q_tutorial_switch.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    let pos = Vec3::new(
        grid_coords.x as f32 * 32.0,
        grid_coords.y as f32 * 32.0,
        0.0,
    );

    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.ui_keys_animations[0].clone()).repeat();

    let transform = Transform::from_translation(pos + SELECTION_KEY_SPAWN_OFFSET);
    commands.spawn((
        animator,
        YSort(-200.0),
        SpriteBundle {
            texture: assets.ui_interact_key_texture.clone(),
            transform,
            ..default()
        },
        TextureAtlas {
            layout: assets.ui_interact_key_layout.clone(),
            ..default()
        },
    ));
}

pub struct KeyboardHintPlugin;

impl Plugin for KeyboardHintPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_keyboard_ui, spawn_selection_key_ui).run_if(in_state(GameState::Gaming)),
        );
    }
}
