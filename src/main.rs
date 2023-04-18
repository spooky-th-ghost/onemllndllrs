use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_vfx_bag::BevyVfxBagPlugin;
use leafwing_input_manager::prelude::InputManagerPlugin;

pub mod camera;
use camera::*;

pub mod movement;
use movement::*;

pub mod input;
use input::*;

pub mod menus;
use menus::*;

pub mod settings;
use settings::*;

pub mod dialogue;
use dialogue::*;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum GameState {
    MainMenu,
    #[default]
    RunAndGun,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSet {
    Camera,
    Movement,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(BevyVfxBagPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(InputManagerPlugin::<PlayerAction>::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec3::Y * -30.0,
            ..default()
        })
        .add_startup_system(setup)
        .add_plugin(PlayerCameraPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(DialoguePlugin)
        .add_plugin(UserSettingsPlugin)
        //.add_plugin(MenusPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(10.0, 0.5, 10.0))),
            material: materials.add(Color::SEA_GREEN.into()),
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            ..default()
        })
        .insert(Collider::cuboid(5.0, 0.25, 5.0))
        .insert(RigidBody::Fixed);
}
