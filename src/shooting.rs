use bevy::prelude::*;
use bevy_rapier3d::prelude::{ExternalImpulse, RapierContext, RigidBody};
use leafwing_input_manager::prelude::*;

use crate::inventory::Belt;
use crate::money::Wallet;
use crate::weapon::{Shot, TriggerMode};
use crate::{GameState, Player, PlayerAction, PlayerSet, PrimaryCamera};

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            Update,
            PlayerSet::Combat.run_if(in_state(GameState::RunAndGun)),
        )
        .add_systems(Update, debug_shooting.in_set(PlayerSet::Combat));
    }
}

pub fn handle_shooting(
    mut commands: Commands,
    mut player_query: Query<&ActionState<PlayerAction>, With<Player>>,
    camera_query: Query<&Transform, With<PrimaryCamera>>,
    mut belt: ResMut<Belt>,
    rapier_context: Res<RapierContext>,
) {
    let camera_transform = camera_query.single();
    let action = player_query.single_mut();

    let mut shot_to_fire = match belt.get_trigger_mode() {
        TriggerMode::Auto => {
            if action.pressed(PlayerAction::Shoot) {
                belt.fire()
            } else {
                None
            }
        }
        TriggerMode::SemiAuto => {
            if action.just_pressed(PlayerAction::Shoot) {
                belt.fire()
            } else {
                None
            }
        }
    };

    if let Some(shot) = shot_to_fire {
        match shot {
            Shot::SingleHitscan {
                base_damage,
                range,
                force_applied,
            } => {}
            Shot::MultiHitscan {
                base_damage,
                range,
                force_applied,
                count,
                spread,
            } => {
                for _ in 0..count {
                    // Cast a ray, randomize the direction slightly based on spread
                }
            }
            Shot::SingleProjectile {
                base_damage,
                range,
                force_applied,
            } => {}
            Shot::MultiProjectile {
                base_damage,
                range,
                force_applied,
                count,
                spread,
            } => {}
        }
    }
    //TODO: Fire the actual projectile
}

pub fn debug_shooting(
    mut commands: Commands,
    player_query: Query<(Entity, &ActionState<PlayerAction>), With<Player>>,
    camera_query: Query<&Transform, With<PrimaryCamera>>,
    cube_query: Query<
        (&Transform, bevy::ecs::query::Has<ExternalImpulse>),
        (With<RigidBody>, Without<Player>),
    >,
    rapier_context: Res<RapierContext>,
) {
    let (player_entity, action) = player_query.single();
    let camera_transform = camera_query.single();

    if action.just_pressed(PlayerAction::Shoot) {
        let ray_origin = camera_transform.translation;
        let ray_dir = camera_transform.forward();
        let max_toi = 100.0;
        let solid = false;
        let filter = bevy_rapier3d::pipeline::QueryFilter {
            exclude_collider: Some(player_entity),
            exclude_rigid_body: Some(player_entity),
            ..default()
        };

        if let Some((entity, intersection)) =
            rapier_context.cast_ray_and_get_normal(ray_origin, ray_dir, max_toi, solid, filter)
        {
            let (hit_transform, has_external) = cube_query.get(entity).unwrap();
            let center_of_mass = hit_transform.translation;
            let impulse = ExternalImpulse::at_point(
                camera_transform.forward() * 100.0,
                intersection.point,
                center_of_mass,
            );

            if has_external {
                commands
                    .entity(entity)
                    .remove::<ExternalImpulse>()
                    .insert(impulse);
            } else {
                commands.entity(entity).insert(impulse);
            }
        }
    }
}
