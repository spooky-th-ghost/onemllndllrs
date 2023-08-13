use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::GameState::RunAndGun), spawn_hud);
    }
}

#[derive(Component)]
pub struct DebtDisplay;

#[derive(Component)]
pub struct AmmoDisplay;

#[derive(Component)]
pub struct WalletDisplay;

#[derive(Component)]
pub struct PhoneDisplay;

fn spawn_hud(mut commands: Commands) {
    commands
        .spawn(TextBundle {
            text: Text {
                sections: vec![
                    TextSection::new(
                        "30",
                        TextStyle {
                            font_size: 48.0,
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "/30",
                        TextStyle {
                            font_size: 48.0,
                            ..default()
                        },
                    ),
                ],
                alignment: TextAlignment::Center,
                ..default()
            },
            style: Style {
                bottom: Val::Percent(0.0),
                position_type: bevy::ui::PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .insert(AmmoDisplay);
}
