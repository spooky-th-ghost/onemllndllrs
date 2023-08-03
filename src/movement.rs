use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    camera::CameraFocus,
    camera::PrimaryCamera,
    input::{InputListenerBundle, PlayerAction},
    GameState, PlayerSet,
};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            Update,
            PlayerSet::Movement.run_if(in_state(GameState::RunAndGun)),
        )
        .add_systems(Startup, spawn_player)
        .add_systems(
            Update,
            (
                get_player_direction,
                rotate_character_to_direction,
                update_character_momentum,
                apply_momentum,
                handle_grounded,
                handle_jumping,
                handle_wall_detection,
            )
                .chain()
                .in_set(PlayerSet::Movement),
        )
        // Disable Physics calc when we leave the gameplay state
        .add_systems(OnExit(GameState::RunAndGun), disable_physics_simulation)
        // Re-enable Physics calc when we enter the gameplay state
        .add_systems(OnEnter(GameState::RunAndGun), re_enable_physics_simulation);
    }
}

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Grounded;

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
            transform: Transform::from_xyz(0.0, 10.0, 0.0),
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
        .insert(Character);
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

fn handle_grounded(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform, bevy::ecs::query::Has<Grounded>), With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    for (entity, transform, is_grounded) in &player_query {
        let ray_origin = transform.translation;
        let ray_dir = Vec3::NEG_Y;
        let max_distance = 1.1;
        let filter = QueryFilter {
            exclude_collider: Some(entity),
            exclude_rigid_body: Some(entity),
            ..default()
        };
        if let Some((_, _)) =
            rapier_context.cast_ray(ray_origin, ray_dir, max_distance, false, filter)
        {
            if !is_grounded {
                commands.entity(entity).insert(Grounded);
            }

            commands.entity(entity).remove::<TouchingWall>();
        } else {
            if is_grounded {
                commands.entity(entity).remove::<Grounded>();
            }
        }
    }
}

fn handle_jumping(
    mut commands: Commands,
    player_query: Query<(Entity, &ActionState<PlayerAction>), (With<Grounded>, With<Player>)>,
) {
    for (entity, action) in &player_query {
        if action.just_pressed(PlayerAction::Jump) {
            commands
                .entity(entity)
                .remove::<ExternalImpulse>()
                .insert(ExternalImpulse {
                    impulse: Vec3::Y * 15.0,
                    ..default()
                });
        }
    }
}

#[derive(Component)]
pub enum TouchingWall {
    Left,
    Right,
}

fn handle_wall_detection(
    mut commands: Commands,
    camera_focus: Res<CameraFocus>,
    player_query: Query<
        (Entity, bevy::ecs::query::Has<TouchingWall>),
        (With<Player>, Without<Grounded>),
    >,
    rapier_context: Res<RapierContext>,
) {
    for (entity, touching_wall) in &player_query {
        let ray_origin = camera_focus.origin();
        let filter = QueryFilter {
            exclude_collider: Some(entity),
            exclude_rigid_body: Some(entity),
            ..default()
        };
        let max_distance = 0.8;

        let mut wall_contact: Option<TouchingWall> = None;

        // Right Ray
        let right_ray_dir = camera_focus.right();
        if let Some((_, _)) =
            rapier_context.cast_ray(ray_origin, right_ray_dir, max_distance, false, filter)
        {
            wall_contact = Some(TouchingWall::Right);
        }

        // Left Ray
        let left_ray_dir = camera_focus.right() * -1.0;
        if let Some((_, _)) =
            rapier_context.cast_ray(ray_origin, left_ray_dir, max_distance, false, filter)
        {
            wall_contact = Some(TouchingWall::Left);
        }

        if touching_wall {
            commands.entity(entity).remove::<TouchingWall>();
        }

        if let Some(wall) = wall_contact {
            commands.entity(entity).insert(wall);
            println!("Touching wall");
        }
    }
}

fn disable_physics_simulation(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.physics_pipeline_active = false;
}

fn re_enable_physics_simulation(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.physics_pipeline_active = true;
}
