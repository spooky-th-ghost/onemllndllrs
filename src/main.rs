use bevy::prelude::*;
use bevy_gltf_components::ComponentsFromGltfPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::InputManagerPlugin;

pub mod camera;

pub mod movement;

pub mod collision;

pub mod input;

pub mod object;

pub mod settings;

pub mod dialogue;

pub mod weapon;

pub mod inventory;

pub mod money;

pub mod audio;

pub mod hud;

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

#[derive(Resource, Default)]
pub struct AssetCache {
    pub check_texture: Handle<Image>,
    pub check_material: Handle<StandardMaterial>,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(InputManagerPlugin::<input::PlayerAction>::default())
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(ComponentsFromGltfPlugin)
        .insert_resource(RapierConfiguration {
            gravity: Vec3::Y * -30.0,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, prep_colliders.run_if(should_prep_colliders))
        .add_plugins((
            hud::HudPlugin,
            camera::PlayerCameraPlugin,
            movement::MovementPlugin,
            //dialogue::DialoguePlugin,
            settings::UserSettingsPlugin,
            shooting::ShootingPlugin,
            audio::AudioPlugin,
            money::MoneyPlugin,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    let check_texture = assets.load("check_texture.png");
    let check_material = materials.add(StandardMaterial {
        base_color_texture: Some(check_texture.clone()),
        ..default()
    });

    // Ground
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(10.0, 0.5, 10.0))),
            material: check_material.clone(),
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            ..default()
        })
        .insert(Collider::cuboid(5.0, 0.25, 5.0))
        .insert(RigidBody::Fixed);
    // Cube Stack
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
            material: check_material.clone(),
            transform: Transform::from_xyz(3.0, 2.0, 3.0),
            ..default()
        })
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(shooting::Shootable)
        .insert(RigidBody::Dynamic);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
            material: check_material.clone(),
            transform: Transform::from_xyz(3.0, 3.5, 3.0),
            ..default()
        })
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(shooting::Shootable)
        .insert(RigidBody::Dynamic);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
            material: check_material.clone(),
            transform: Transform::from_xyz(3.0, 5.0, 3.0),
            ..default()
        })
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(shooting::Shootable)
        .insert(RigidBody::Dynamic);

    // Scene
    commands.spawn(SceneBundle {
        scene: assets.load("blocks.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, -5.0),
        ..Default::default()
    });
    commands.insert_resource(AssetCache {
        check_texture,
        check_material,
    });
}

#[derive(Component)]
pub struct ColliderPrepped;

fn should_prep_colliders(
    cube_query: Query<&Name, Without<ColliderPrepped>>,
    clip_query: Query<&Name, Without<shooting::ClipComponent>>,
) -> bool {
    let mut needs_prep = false;
    let cube_regex = regex::Regex::new(r"^Cube").unwrap();
    let clip_regex = regex::Regex::new(r"^Clip").unwrap();

    for name in &clip_query {
        if clip_regex.is_match(name.as_str()) {
            needs_prep = true;
        }
    }

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
    clip_query: Query<(&Name, Entity), Without<shooting::ClipComponent>>,
    meshes: Res<Assets<Mesh>>,
    asset_cache: Res<AssetCache>,
) {
    let cube_regex = regex::Regex::new(r"^Cube").unwrap();
    let clip_regex = regex::Regex::new(r"^Clip").unwrap();
    for (name, entity) in &clip_query {
        if clip_regex.is_match(name.as_str()) {
            commands.entity(entity).insert(shooting::ClipComponent);
        }
    }
    for (handle, name, entity) in &cube_query {
        if cube_regex.is_match(name.as_str()) {
            if let Some(mesh) = meshes.get(handle) {
                let collider =
                    Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap();
                commands
                    .entity(entity)
                    .remove::<Handle<StandardMaterial>>()
                    .insert((
                        collider,
                        RigidBody::Fixed,
                        ColliderPrepped,
                        asset_cache.check_material.clone(),
                    ));
            }
        }
    }
}
