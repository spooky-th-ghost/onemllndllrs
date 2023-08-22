use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    camera::PrimaryCamera,
    input::{InputListenerBundle, PlayerAction},
    movement::{Character, Direction, Momentum, Movespeed},
    GameState,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::GameState::RunAndGun), spawn_player)
            .add_systems(
                Update,
                get_player_direction.run_if(in_state(GameState::RunAndGun)),
            );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerStats {
    entity: Entity,
    interacting_object: Option<Entity>,
    held_object: Option<Entity>,
}

impl PlayerStats {
    pub fn new(entity: Entity) -> Self {
        PlayerStats {
            entity,
            interacting_object: None,
            held_object: None,
        }
    }
    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn clear_interacted(&mut self) {
        self.interacting_object = None;
    }

    pub fn set_interacted(&mut self, entity: Entity) {
        self.interacting_object = Some(entity);
    }
}

fn spawn_player(mut commands: Commands) {
    let player_entity = commands
        .spawn(TransformBundle {
            local: Transform::from_xyz(0.0, 10.0, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::capsule_y(0.5, 0.5))
        .insert(Damping {
            linear_damping: 0.2,
            angular_damping: 0.0,
        })
        .insert(InputListenerBundle::input_map())
        .insert(Friction {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(GravityScale(1.0))
        .insert(Direction::default())
        .insert(Momentum::default())
        .insert(Movespeed::default())
        .insert(Player)
        .insert(Name::new("Player"))
        .insert(Character)
        .id();

    commands.insert_resource(PlayerStats::new(player_entity));
}

fn get_player_direction(
    mut player_query: Query<(&mut Direction, &ActionState<PlayerAction>), With<Player>>,
    camera_query: Query<&Transform, With<PrimaryCamera>>,
) {
    let camera_transform = camera_query.single();
    let (mut direction, action) = player_query.single_mut();

    let forward = Vec3::new(
        camera_transform.forward().x,
        0.0,
        camera_transform.forward().z,
    )
    .normalize_or_zero();

    let right =
        Vec3::new(camera_transform.right().x, 0.0, camera_transform.right().z).normalize_or_zero();
    if action.pressed(PlayerAction::Move) {
        let axis_pair = action.clamped_axis_pair(PlayerAction::Move).unwrap();

        direction.set((axis_pair.y() * forward) + (axis_pair.x() * right));
    } else {
        direction.reset();
    }
}
