use crate::{GameState, Player, PlayerAction, PlayerSet};
use bevy::prelude::*;
use bevy_vfx_bag::post_processing::pixelate::Pixelate;
use leafwing_input_manager::prelude::*;

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(PlayerSet::Camera.in_set(OnUpdate(GameState::RunAndGun)))
            .add_startup_system(spawn_camera)
            .add_systems(
                (
                    read_rotation_inputs_primary,
                    switch_camera_perspective,
                    target_player,
                    position_and_rotate_camera,
                )
                    .chain(),
            );
    }
}

pub enum CameraMode {
    Explore,
    Shoot,
    Cutscene,
}

pub enum CameraPerspective {
    FirstPerson,
    ThirdPerson,
}

#[derive(Component)]
pub struct PrimaryCamera {
    pub offset: Vec3,
    pub x_angle: f32,
    pub y_angle: f32,
    pub target: Vec3,
    pub mode: CameraMode,
    pub perspective: CameraPerspective,
}

impl PrimaryCamera {
    pub fn adjust_x_angle(&mut self, increase: f32) {
        let max_x = match self.perspective {
            CameraPerspective::FirstPerson => 87.0,
            CameraPerspective::ThirdPerson => 20.0,
        };

        let min_x = match self.perspective {
            CameraPerspective::FirstPerson => -87.0,
            CameraPerspective::ThirdPerson => -2.0,
        };

        self.x_angle = (self.x_angle + increase).clamp(min_x, max_x);
    }

    pub fn adjust_y_angle(&mut self, increase: f32) {
        self.y_angle += increase;
    }
}

impl Default for PrimaryCamera {
    fn default() -> Self {
        PrimaryCamera {
            offset: Vec3::new(-1.0, 0.5, -6.0),
            x_angle: 0.0,
            y_angle: 0.0,
            target: Vec3::ZERO,
            mode: CameraMode::Shoot,
            perspective: CameraPerspective::FirstPerson,
        }
    }
}

fn switch_camera_perspective(
    mut camera_query: Query<&mut PrimaryCamera>,
    player_query: Query<&ActionState<PlayerAction>>,
) {
    let mut camera = camera_query.single_mut();
    let action = player_query.single();

    if action.just_pressed(PlayerAction::SwitchPerspective) {
        camera.perspective = match camera.perspective {
            CameraPerspective::FirstPerson => CameraPerspective::ThirdPerson,
            CameraPerspective::ThirdPerson => CameraPerspective::FirstPerson,
        };
    }
}

fn read_rotation_inputs_primary(
    mut camera_query: Query<&mut PrimaryCamera>,
    player_query: Query<&ActionState<PlayerAction>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();
    let action = player_query.single();

    if action.pressed(PlayerAction::Pan) {
        let camera_pan_vector = action.axis_pair(PlayerAction::Pan).unwrap();

        let y_rot_change = if camera_pan_vector.x() != 0.0 {
            15.0 * camera_pan_vector.x() * time.delta_seconds()
        } else {
            0.0
        };
        let x_rot_change = if camera_pan_vector.y() != 0.0 {
            15.0 * camera_pan_vector.y() * time.delta_seconds()
        } else {
            0.0
        };
        if x_rot_change != 0.0 {
            camera.adjust_x_angle(-x_rot_change);
        }
        if y_rot_change != 0.0 {
            camera.adjust_y_angle(-y_rot_change);
        }
    }

    if action.pressed(PlayerAction::PanGamepad) {
        let camera_pan_vector = action.axis_pair(PlayerAction::PanGamepad).unwrap();

        let y_rot_change = if camera_pan_vector.x() != 0.0 {
            180.0 * camera_pan_vector.x() * time.delta_seconds()
        } else {
            0.0
        };
        let x_rot_change = if camera_pan_vector.y() != 0.0 {
            90.0 * camera_pan_vector.y() * time.delta_seconds()
        } else {
            0.0
        };
        if x_rot_change != 0.0 {
            camera.adjust_x_angle(x_rot_change);
        }
        if y_rot_change != 0.0 {
            camera.adjust_y_angle(-y_rot_change);
        }
    }
}

fn target_player(
    mut camera_query: Query<&mut PrimaryCamera, Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let mut camera = camera_query.single_mut();
    let player_transform = player_query.single();

    camera.target = player_transform.translation;
}

fn position_and_rotate_camera(
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &PrimaryCamera)>,
) {
    let (mut transform, camera) = camera_query.single_mut();
    let mut starting_transform = Transform::from_translation(camera.target);
    let x_angle = camera.x_angle.to_radians();
    let y_angle = camera.y_angle.to_radians();

    starting_transform.rotate_y(y_angle);

    let forward = starting_transform.forward().normalize();
    let right = starting_transform.right().normalize();

    let desired_position = match camera.perspective {
        CameraPerspective::ThirdPerson => {
            starting_transform.translation
                + (forward * camera.offset.z)
                + (right * camera.offset.x)
                + (Vec3::Y * camera.offset.y)
        }
        CameraPerspective::FirstPerson => {
            starting_transform.translation + (Vec3::Y * camera.offset.y)
        }
    };

    let mut desired_rotatation = Transform::default();

    desired_rotatation.rotate_x(x_angle);
    desired_rotatation.rotate_y(y_angle);

    let slerp_rotation = transform
        .rotation
        .slerp(desired_rotatation.rotation, time.delta_seconds() * 20.0);
    let lerp_position = transform
        .translation
        .lerp(desired_position, time.delta_seconds() * 20.0);

    transform.translation = lerp_position;
    transform.rotation = slerp_rotation;
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(PrimaryCamera::default())
        .insert(Pixelate { block_size: 2.0 });
}
