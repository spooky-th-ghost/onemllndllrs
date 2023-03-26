use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{GameState, InputListenerBundle, PlayerAction, PlayerSet, PrimaryCamera};

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(PlayerSet::Movement.in_set(OnUpdate(GameState::RunAndGun)))
            .add_startup_system(spawn_player)
            .add_systems(
                (
                    get_player_direction,
                    rotate_player_to_direction,
                    move_player,
                )
                    .chain(),
            );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct Movement {
    pub direction: Vec3,
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(Color::MIDNIGHT_BLUE.into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
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
        .insert(Movement::default())
        .insert(Player)
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(0.5, 0.5, 0.5))),
                material: materials.add(Color::WHITE.into()),
                transform: Transform::from_xyz(0.0, 0.5, -0.5),
                ..default()
            });
        });
}

fn get_player_direction(
    mut player_query: Query<(&mut Movement, &ActionState<PlayerAction>), With<Player>>,
    camera_query: Query<&Transform, With<PrimaryCamera>>,
) {
    let camera_transform = camera_query.single();
    let (mut movement, action) = player_query.single_mut();

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

        movement.direction =
            ((axis_pair.y() * forward) + (axis_pair.x() * right)).normalize_or_zero();
    } else {
        movement.direction = Vec3::ZERO;
    }
}

fn rotate_player_to_direction(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Movement), With<Player>>,
    mut rotation_target: Local<Transform>,
) {
    for (mut transform, movement) in &mut player_query {
        rotation_target.translation = transform.translation;
        let flat_velo_direction =
            Vec3::new(movement.direction.x, 0.0, movement.direction.z).normalize_or_zero();
        if flat_velo_direction != Vec3::ZERO {
            let target_position = rotation_target.translation + flat_velo_direction;

            rotation_target.look_at(target_position, Vec3::Y);
            let turn_speed = 15.0;

            transform.rotation = transform
                .rotation
                .slerp(rotation_target.rotation, time.delta_seconds() * turn_speed);
        }
    }
}

fn move_player(mut player_query: Query<(&mut Velocity, &Transform, &Movement), With<Player>>) {
    for (mut velocity, transform, movement) in &mut player_query {
        if movement.direction != Vec3::ZERO {
            velocity.linvel = transform.forward().normalize_or_zero() * 2.0;
        }
    }
}
