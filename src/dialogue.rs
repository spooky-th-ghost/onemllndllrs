use std::time::Duration;

use bevy::prelude::*;

use crate::settings::DisplaySettings;

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_text_box)
            .add_systems(Update, update_dialogue);
    }
}

#[derive(Resource)]
pub struct Dialogue {
    current_dialogue: String,
    character_index: usize,
    character_timer: Timer,
}

#[derive(Component)]
pub struct TextBox;

impl Dialogue {
    pub fn tick(&mut self, delta: Duration) {
        if self.character_index < self.current_dialogue.len() {
            self.character_timer.tick(delta);
            if self.character_timer.just_finished() {
                self.character_timer.reset();
                self.character_index += 1;
            }
        }
    }
}

impl std::fmt::Display for Dialogue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.current_dialogue[..self.character_index].to_owned()
        )
    }
}

pub fn setup_text_box(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    display_settings: Res<DisplaySettings>,
) {
    let resolution = display_settings.resolution;
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(90.0),
                height: Val::Percent(30.0),
                left: Val::Percent(5.0),
                bottom: Val::Percent(-70.0),
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.4).into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/Alexandria.ttf"),
                            font_size: 50.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::all(Val::Percent(2.5)),
                        max_width: Val::Px(resolution.0 * 0.88),
                        ..default()
                    }),
                )
                .insert(TextBox);
        });

    commands.insert_resource(Dialogue {
        current_dialogue: "This is some test dialogue, hope it looks good, who knows, making it longer to see if it wraps correctly, so hopefully this isn't running off screen, that's good right?".to_string(),
        character_index: 0,
        character_timer: Timer::from_seconds(0.025, TimerMode::Once),
    });
}

pub fn update_dialogue(
    mut dialogue: ResMut<Dialogue>,
    time: Res<Time>,
    mut query: Query<&mut Text, With<TextBox>>,
) {
    dialogue.tick(time.delta());
    for mut text in &mut query {
        text.sections[0].value = dialogue.to_string();
    }
}
