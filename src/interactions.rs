use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Interactable(pub InteractionType);

pub enum InteractionType {
    Talk,
    Pickup,
}

#[derive(Event)]
pub struct PickupEvent {
    pub object_entity: Entity,
    pub holder_entity: Entity,
}

impl std::fmt::Display for InteractionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InteractionType::Talk => {
                write!(f, "Talk")
            }
            InteractionType::Pickup => {
                write!(f, "Pick-up")
            }
        }
    }
}

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            set_interaction_state.run_if(in_state(crate::GameState::RunAndGun)),
        );
    }
}

fn set_interaction_state(
    mut interaction_display_query: Query<
        (&mut Visibility, &mut Text),
        With<crate::hud::InteractDisplay>,
    >,
    player_query: Query<Entity, With<crate::player::Player>>,
    interactable_query: Query<&Interactable>,
    mut player_stats: ResMut<crate::player::PlayerStats>,
    camera_focus: Res<crate::camera::CameraFocus>,
    rapier_context: Res<RapierContext>,
) {
    if let Ok((mut visibility, mut text)) = interaction_display_query.get_single_mut() {
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
                if let Ok(inteactable) = interactable_query.get(entity) {
                    *visibility = Visibility::Visible;
                    text.sections[1].value = inteactable.0.to_string();
                    player_stats.set_interacted(entity);
                } else {
                    *visibility = Visibility::Hidden;
                    player_stats.clear_interacted();
                }
            } else {
                *visibility = Visibility::Hidden;
                player_stats.clear_interacted();
            }
        }
    }
}
