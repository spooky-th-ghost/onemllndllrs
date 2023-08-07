use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hud);
    }
}

#[derive(Component)]
pub struct AmmoDisplay;

fn spawn_hud(mut commands: Commands) {
    // commands
    //     .spawn(NodeBundle {
    //         style: Style {
    //             display: Display::Grid,
    //             grid_template_rows: RepeatedGridTrack::percent(4, 0.25),
    //             grid_template_columns: RepeatedGridTrack::percent(4, 0.25),
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .with_children(|parent| {
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
    // });
}
