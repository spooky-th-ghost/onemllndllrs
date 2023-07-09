use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::inventory::Belt;
use crate::money::Wallet;
use crate::weapon::{Shot, TriggerMode};
use crate::{GameState, Player, PlayerAction, PlayerSet, PrimaryCamera};

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(PlayerSet::Combat.in_set(OnUpdate(GameState::RunAndGun)));
    }
}

pub fn handle_shooting(
    mut commands: Commands,
    mut player_query: Query<&ActionState<PlayerAction>, With<Player>>,
    camera_query: Query<&Transform, With<PrimaryCamera>>,
    mut belt: ResMut<Belt>,
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
    //TODO: Fire the actual projectile
}
