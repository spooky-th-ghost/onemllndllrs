use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::GameState::RunAndGun), spawn_hud);
    }
}

#[derive(Component)]
pub struct InteractDisplay;

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
                        "<[E]>\n",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::YELLOW,
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "Interact",
                        TextStyle {
                            font_size: 24.0,
                            ..default()
                        },
                    ),
                ],
                alignment: TextAlignment::Center,
                ..default()
            },
            style: Style {
                top: Val::Percent(60.0),
                left: Val::Percent(50.0),
                ..default()
            },
            ..default()
        })
        .insert(InteractDisplay);
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

    commands.spawn(Camera3dBundle {
        camera_3d: Camera3d {
            clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::None,
            ..default()
        },
        camera: Camera {
            order: 4,
            ..default()
        },
        transform: Transform::from_xyz(0., 1000., 16.).looking_at(Vec3::Y * 1000.0, Vec3::Y),
        ..default()
    });
}
