use std::time::Duration;

use bevy::prelude::*;

use crate::{GameAssets, GameState};

#[derive(Component)]
struct WriteableText {
    written_text: String,
    text_left: String,
    timer: Timer,
    time_between_chars: f32,
}

impl WriteableText {
    fn new(text_left: &str, time_between_chars: f32, start_delay: f32) -> Self {
        Self {
            written_text: String::new(),
            text_left: text_left.to_string(),
            timer: Timer::from_seconds(start_delay, TimerMode::Repeating),
            time_between_chars,
        }
    }
}

fn text_style_standard(assets: &Res<GameAssets>) -> TextStyle {
    TextStyle {
        font_size: 100.0,
        color: Color::WHITE,
        font: assets.silver_font.clone(),
    }
}

fn spawn_fin_text(commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
    commands
        .spawn((
            TextBundle {
                text: Text::from_section("", text_style_standard(assets)),
                z_index: ZIndex::Local(1),
                ..default()
            },
            WriteableText::new("FIN", 0.25, 1.5),
        ))
        .id()
}

fn spawn_thanks_text(commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
    commands
        .spawn((
            TextBundle {
                text: Text::from_section("", text_style_standard(assets)),
                z_index: ZIndex::Local(1),
                ..default()
            },
            WriteableText::new("Thanks for Playing", 0.1, 5.0),
        ))
        .id()
}

fn spawn_final_texts(mut commands: Commands, assets: Res<GameAssets>) {
    let header_text = spawn_fin_text(&mut commands, &assets);
    let body_text = spawn_thanks_text(&mut commands, &assets);

    commands
        .spawn((NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Percent(40.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Vh(25.0),
                align_items: AlignItems::Center,
                ..default()
            },
            z_index: ZIndex::Global(1001),
            ..default()
        },))
        .push_children(&[header_text, body_text]);
}

fn write_texts(time: Res<Time>, mut q_writable_texts: Query<(&mut Text, &mut WriteableText)>) {
    for (mut text, mut writeable_text) in &mut q_writable_texts {
        writeable_text.timer.tick(time.delta());
        if writeable_text.timer.just_finished() {
            if writeable_text.text_left.is_empty() {
                writeable_text.timer.set_mode(TimerMode::Once);
                continue;
            }

            // This was the start delay
            if writeable_text.written_text.is_empty() {
                let duration = Duration::from_secs_f32(writeable_text.time_between_chars);
                writeable_text.timer.set_duration(duration);
            }

            let next_char = writeable_text.text_left.remove(0);
            writeable_text.written_text.push(next_char);
        }
        text.sections[0]
            .value
            .clone_from(&writeable_text.written_text);
    }
}

pub struct EndingTextPlugin;

impl Plugin for EndingTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            write_texts.run_if(not(in_state(GameState::AssetLoading))),
        )
        .add_systems(OnEnter(GameState::GameOver), spawn_final_texts);
    }
}
