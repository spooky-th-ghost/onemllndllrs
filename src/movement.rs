use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{GameState, InputListenerBundle, PlayerAction, PlayerSet, PrimaryCamera};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(PlayerSet::Movement.in_set(OnUpdate(GameState::RunAndGun)))
            .add_startup_system(spawn_player)
            .add_systems(
                (
                    get_player_direction,
                    rotate_character_to_direction,
                    update_character_momentum,
                    apply_momentum,
                )
                    .chain(),
            );
    }
}

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Strafe;

#[derive(Component)]
pub struct Movespeed(f32);

impl Movespeed {
    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn set(&mut self, value: f32) {
        self.0 = value;
    }
}

impl Default for Movespeed {
    fn default() -> Self {
        Movespeed(5.0)
    }
}

#[derive(Component, Default)]
pub struct Direction(Vec3);

impl Direction {
    pub fn get(&self) -> Vec3 {
        self.0
    }

    pub fn has_some(&self) -> bool {
        self.0 != Vec3::ZERO
    }

    pub fn set(&mut self, value: Vec3) {
        self.0 = value.normalize_or_zero();
    }

    pub fn reset(&mut self) {
        self.0 = Vec3::ZERO;
    }
}

#[derive(Component, Default)]
pub struct Momentum(Vec3);

impl Momentum {
    pub fn get(&self) -> Vec3 {
        self.0
    }

    pub fn has_some(&self) -> bool {
        self.0 != Vec3::ZERO
    }

    pub fn set(&mut self, value: Vec3) {
        self.0 = value;
    }

    pub fn reset(&mut self) {
        self.0 = Vec3::ZERO;
    }
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
        .insert(Direction::default())
        .insert(Momentum::default())
        .insert(Movespeed::default())
        .insert(Player)
        .insert(Character)
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

fn rotate_character_to_direction(
    time: Res<Time>,
    mut character_query: Query<(&mut Transform, &Direction), (With<Character>, Without<Strafe>)>,
    mut rotation_target: Local<Transform>,
) {
    for (mut transform, direction) in &mut character_query {
        rotation_target.translation = transform.translation;
        let dir = direction.get();
        let flat_velo_direction = Vec3::new(dir.x, 0.0, dir.z).normalize_or_zero();
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

fn update_character_momentum(
    mut character_query: Query<(&mut Momentum, &Movespeed, &Direction), With<Character>>,
) {
    for (mut momentum, movespeed, direction) in &mut character_query {
        if direction.has_some() {
            momentum.set(direction.get() * movespeed.get());
        } else {
            momentum.reset();
        }
    }
}

fn apply_momentum(mut query: Query<(&mut Velocity, &Momentum)>) {
    for (mut velocity, momentum) in &mut query {
        let mut velocity_to_apply = Vec3::ZERO;
        let mut should_change_velocity: bool = false;

        if momentum.has_some() {
            should_change_velocity = true;
            velocity_to_apply = momentum.get();
        }

        if should_change_velocity {
            velocity.linvel.x = velocity_to_apply.x;
            velocity.linvel.z = velocity_to_apply.z;
        }
    }
}
