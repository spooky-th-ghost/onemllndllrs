use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::InputManagerPlugin;

pub mod camera;
use camera::*;

pub mod movement;
use movement::*;

pub mod collision;
use collision::*;

pub mod input;
use input::*;

pub mod object;
use object::*;

pub mod settings;
use settings::*;

pub mod dialogue;
use dialogue::*;

pub mod weapon;

pub mod inventory;

pub mod money;

pub mod shooting;

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
    Combat,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .add_plugins(WorldInspectorPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec3::Y * -30.0,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, prep_colliders.run_if(should_prep_colliders))
        .add_plugins((
            PlayerCameraPlugin,
            MovementPlugin,
            DialoguePlugin,
            UserSettingsPlugin,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    // Ground
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(10.0, 0.5, 10.0))),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(assets.load("check_texture.png")),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            ..default()
        })
        .insert(Collider::cuboid(5.0, 0.25, 5.0))
        .insert(RigidBody::Fixed);

    // Scene
    commands.spawn(SceneBundle {
        scene: assets.load("blocks.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, -5.0),
        ..Default::default()
    });
}

#[derive(Component)]
pub struct ColliderPrepped;

fn should_prep_colliders(cube_query: Query<&Name, Without<ColliderPrepped>>) -> bool {
    let mut needs_prep = false;
    let cube_regex = regex::Regex::new(r"^Cube").unwrap();
    for name in &cube_query {
        if cube_regex.is_match(name.as_str()) {
            needs_prep = true;
        }
    }
    if needs_prep {
        println!("Prepping Colliders");
    }
    needs_prep
}

fn prep_colliders(
    mut commands: Commands,
    cube_query: Query<(&Handle<Mesh>, &Name, Entity), Without<ColliderPrepped>>,
    meshes: Res<Assets<Mesh>>,
) {
    let cube_regex = regex::Regex::new(r"^Cube").unwrap();
    for (handle, name, entity) in &cube_query {
        if cube_regex.is_match(name.as_str()) {
            if let Some(mesh) = meshes.get(handle) {
                let collider =
                    Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap();
                commands
                    .entity(entity)
                    .insert((collider, RigidBody::Fixed, ColliderPrepped));
            }
        }
    }
}

fn move_starting_platform(time: Res<Time>, mut query: Query<(&mut Transform, &Name)>) {
    for (mut transform, name) in &mut query {
        if name.as_str() == "Starting Platform" {
            transform.translation.y += 0.1 * time.delta_seconds();
        }
    }
}
