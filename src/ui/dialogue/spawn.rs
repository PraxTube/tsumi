use bevy::prelude::*;
use bevy_tweening::{lens::*, *};

use crate::{
    aspect::CombinedAspect,
    npc::narrator::TriggeredNarratorDialogue,
    world::{PlayerWentToBed, TriggerFirstImaDialogue},
    GameAssets, GameState,
};

// The master root of the dialogue
#[derive(Component)]
pub struct DialogueRoot;
#[derive(Component)]
pub struct DialogueContent;
#[derive(Component)]
pub struct DialogueNameNode;
#[derive(Component)]
pub struct DialogueCharacterIcon;
#[derive(Component)]
pub struct DialogueContinueNode;

const DIALOG_WIDTH: f32 = 800.0 * 0.8;
const TEXT_BORDER: f32 = 120.0;

const CONTINUE_BOTTOM: f32 = -5.0;
const CONTINUE_BOB_DURATION: f32 = 1.0;
const CONTINUE_BOB_OFFSET: f32 = 5.0;

fn style_standard(_assets: &Res<GameAssets>) -> Style {
    Style {
        max_width: Val::Px(DIALOG_WIDTH - 2.0 * TEXT_BORDER),
        ..default()
    }
}

fn text_style_standard(assets: &Res<GameAssets>) -> TextStyle {
    TextStyle {
        font: assets.silver_font.clone(),
        font_size: 50.0,
        color: Color::WHITE,
    }
}

fn text_style_name(assets: &Res<GameAssets>) -> TextStyle {
    TextStyle {
        font: assets.silver_font.clone(),
        font_size: 46.0,
        color: Color::WHITE,
    }
}

fn spawn_dialogue_top(commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
    let edge = commands
        .spawn((ImageBundle {
            image: UiImage {
                texture: assets.dialogue_edge.clone(),
                ..default()
            },
            style: Style {
                width: Val::Px(DIALOG_WIDTH),
                ..default()
            },
            ..default()
        },))
        .id();

    let name_node = commands
        .spawn((
            TextBundle {
                text: Text::from_section(String::new(), text_style_name(assets)),
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(TEXT_BORDER / 2.0),
                    top: Val::Px(-8.0),
                    ..default()
                },
                z_index: ZIndex::Local(1),
                ..default()
            },
            DialogueNameNode,
            Label,
        ))
        .id();

    commands
        .spawn((NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },))
        .push_children(&[edge, name_node])
        .id()
}

fn spawn_dialogue_content(commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
    let text = commands
        .spawn((
            DialogueContent,
            Label,
            TextBundle::from_section(String::new(), text_style_standard(assets))
                .with_style(style_standard(assets)),
        ))
        .id();

    let icon = commands
        .spawn((
            DialogueCharacterIcon,
            ImageBundle {
                style: Style {
                    width: Val::Px(128.0),
                    left: Val::Px(-128.0 - 16.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Px(DIALOG_WIDTH),
                min_height: Val::Px(50.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::FlexStart,
                padding: UiRect::horizontal(Val::Px(TEXT_BORDER)),
                ..default()
            },
            background_color: Color::BLACK.with_alpha(0.8).into(),
            ..default()
        },))
        .push_children(&[text, icon])
        .id()
}

fn spawn_dialogue_bottom(commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
    let edge = commands
        .spawn((ImageBundle {
            image: UiImage {
                texture: assets.dialogue_edge.clone(),
                flip_y: true,
                ..default()
            },
            style: Style {
                width: Val::Px(DIALOG_WIDTH),
                ..default()
            },
            ..default()
        },))
        .id();

    let tween = Tween::new(
        EaseFunction::SineInOut,
        std::time::Duration::from_secs_f32(CONTINUE_BOB_DURATION),
        UiPositionLens {
            start: UiRect::new(
                Val::Auto,
                Val::Auto,
                Val::Auto,
                Val::Px(CONTINUE_BOTTOM + CONTINUE_BOB_OFFSET),
            ),
            end: UiRect::new(
                Val::Auto,
                Val::Auto,
                Val::Auto,
                Val::Px(CONTINUE_BOTTOM - CONTINUE_BOB_OFFSET),
            ),
        },
    )
    .with_repeat_count(RepeatCount::Infinite)
    .with_repeat_strategy(RepeatStrategy::MirroredRepeat);

    let continue_node = commands
        .spawn((
            DialogueContinueNode,
            Animator::new(tween),
            ImageBundle {
                image: UiImage {
                    texture: assets.dialogue_continue.clone(),
                    ..default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                z_index: ZIndex::Local(1),
                visibility: Visibility::Hidden,
                ..default()
            },
        ))
        .id();

    commands
        .spawn((NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },))
        .push_children(&[edge, continue_node])
        .id()
}

fn spawn_dialogue(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    justify_content: JustifyContent,
) {
    let dialogue_top = spawn_dialogue_top(commands, assets);
    let dialogue_content = spawn_dialogue_content(commands, assets);
    let dialogue_bottom = spawn_dialogue_bottom(commands, assets);

    let dialogue_root = commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                padding: UiRect::bottom(Val::Px(30.0)),
                flex_direction: FlexDirection::Column,
                justify_content,
                ..default()
            },
            ..default()
        },))
        .push_children(&[dialogue_top, dialogue_content, dialogue_bottom])
        .id();

    commands
        .spawn((
            DialogueRoot,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                z_index: ZIndex::Global(100),
                ..default()
            },
        ))
        .push_children(&[dialogue_root]);
}

fn spawn_npc_dialogue(mut commands: Commands, assets: Res<GameAssets>) {
    spawn_dialogue(&mut commands, &assets, JustifyContent::FlexEnd);
}

fn spawn_narrator_dialogue(mut commands: Commands, assets: Res<GameAssets>) {
    spawn_dialogue(&mut commands, &assets, JustifyContent::Center);
}

pub fn create_dialogue_text(
    text: impl Into<String>,
    invisible: impl Into<String>,
    assets: &Res<GameAssets>,
) -> Text {
    Text::from_sections([
        TextSection {
            value: text.into(),
            style: text_style_standard(assets),
        },
        TextSection {
            value: invisible.into(),
            style: TextStyle {
                color: Color::NONE,
                ..text_style_standard(assets)
            },
        },
    ])
}

pub struct DialogueSpawnPlugin;

impl Plugin for DialogueSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_npc_dialogue.run_if(on_event::<CombinedAspect>().or_else(
                    on_event::<TriggerFirstImaDialogue>().or_else(on_event::<PlayerWentToBed>()),
                )),
                spawn_narrator_dialogue.run_if(on_event::<TriggeredNarratorDialogue>()),
            )
                .run_if(not(in_state(GameState::AssetLoading))),
        );
    }
}
