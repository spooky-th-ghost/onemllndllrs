use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hud);
    }
}

#[derive(Component)]
pub struct DebtDisplay;

#[derive(Component)]
pub struct AmmoDisplay;

#[derive(Component)]
pub struct WalletDisplay;

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
                left: Val::Percent(-90.0),
                bottom: Val::Percent(-94.0),
                ..default()
            },
            ..default()
        })
        .insert(AmmoDisplay);

    commands
        .spawn(TextBundle {
            text: Text {
                sections: vec![
                    TextSection::new(
                        "Dosh: ",
                        TextStyle {
                            font_size: 48.0,
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "$0.00",
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
                left: Val::Percent(-90.0),
                bottom: Val::Percent(-10.0),
                ..default()
            },
            ..default()
        })
        .insert(WalletDisplay);
}
