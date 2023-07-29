use bevy::prelude::*;
use bevy_rapier3d::prelude::RapierContext;
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
        );
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
