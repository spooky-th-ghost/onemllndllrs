use bevy::prelude::*;
use bevy_rapier3d::prelude::RapierContext;

use crate::{camera::CameraFocus, movement::Player};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::GameState::RunAndGun), spawn_hud)
            .add_systems(
                Update,
                show_interaction_popup.run_if(in_state(crate::GameState::RunAndGun)),
            );
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
                        "<[E]>",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::YELLOW,
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "\nInteract",
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
}

fn show_interaction_popup(
    mut interaction_display_query: Query<&mut Visibility, With<InteractDisplay>>,
    player_query: Query<Entity, With<Player>>,
    interactable_query: Query<Entity, With<crate::Interactable>>,
    camera_focus: Res<CameraFocus>,
    rapier_context: Res<RapierContext>,
) {
    if let Ok(mut visibility) = interaction_display_query.get_single_mut() {
        if let Ok(player_entity) = player_query.get_single() {
            let ray_origin = camera_focus.origin();
            let ray_dir = camera_focus.forward();
            let max_distance = 2.0;
            let solid = false;
            let filter = bevy_rapier3d::pipeline::QueryFilter {
                exclude_collider: Some(player_entity),
                exclude_rigid_body: Some(player_entity),
                ..default()
            };

            if let Some((entity, _distance)) =
                rapier_context.cast_ray(ray_origin, ray_dir, max_distance, solid, filter)
            {
                if interactable_query.contains(entity) {
                    *visibility = Visibility::Visible;
                } else {
                    *visibility = Visibility::Hidden;
                }
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}
