use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;
use bevy_gltf_components::ComponentsFromGltfPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::InputManagerPlugin;

pub mod interactions;

pub mod camera;

pub mod clock;

pub mod movement;

pub mod collision;

pub mod input;

pub mod object;

pub mod settings;

pub mod dialogue;

pub mod weapon;

pub mod inventory;

pub mod player;

pub mod money;

pub mod audio;

pub mod hud;

pub mod phone;

pub mod shooting;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    RunAndGun,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSet {
    Camera,
    Movement,
    Combat,
}

#[derive(Resource, AssetCollection)]
pub struct AssetCache {
    #[asset(path = "check_texture.png", standard_material)]
    pub check_material: Handle<StandardMaterial>,
    #[asset(path = "phone.glb#Scene0")]
    pub phone: Handle<Scene>,
    #[asset(path = "stool.glb#Scene0")]
    pub stool: Handle<Scene>,
    pub screen_material: Handle<StandardMaterial>,
    #[asset(paths("buildings/teahouse.glb#Scene0"), collection(typed, mapped))]
    pub buildings: HashMap<String, Handle<Scene>>,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::RunAndGun),
        )
        .add_collection_to_loading_state::<_, AssetCache>(GameState::Loading)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(InputManagerPlugin::<input::PlayerAction>::default())
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(ComponentsFromGltfPlugin)
        .insert_resource(RapierConfiguration {
            gravity: Vec3::Y * -30.0,
            ..default()
        })
        .add_systems(OnEnter(GameState::RunAndGun), setup)
        .add_systems(
            Update,
            prep_colliders
                .run_if(should_prep_colliders)
                .run_if(in_state(GameState::RunAndGun)),
        )
        .add_plugins((
            hud::HudPlugin,
            clock::ClockPlugin,
            camera::PlayerCameraPlugin,
            movement::MovementPlugin,
            //dialogue::DialoguePlugin,
            interactions::InteractionsPlugin,
            player::PlayerPlugin,
            settings::UserSettingsPlugin,
            shooting::ShootingPlugin,
            audio::AudioPlugin,
            money::MoneyPlugin,
            phone::PhonePlugin,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_cache: Res<AssetCache>,
    assets: Res<AssetServer>,
) {
    // Ambient Light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });

    // Ground
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(10.0, 0.5, 10.0))),
            material: asset_cache.check_material.clone(),
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            ..default()
        })
        .insert(Collider::cuboid(5.0, 0.25, 5.0))
        .insert(RigidBody::Fixed);
    // Cube Stack
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
            material: asset_cache.check_material.clone(),
            transform: Transform::from_xyz(3.0, 2.0, 3.0),
            ..default()
        })
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(shooting::Shootable)
        .insert(RigidBody::Dynamic)
        .insert(interactions::Interactable(
            interactions::InteractionType::Pickup,
        ));
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
            material: asset_cache.check_material.clone(),
            transform: Transform::from_xyz(3.0, 3.5, 3.0),
            ..default()
        })
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(shooting::Shootable)
        .insert(RigidBody::Dynamic)
        .insert(interactions::Interactable(
            interactions::InteractionType::Pickup,
        ));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
            material: asset_cache.check_material.clone(),
            transform: Transform::from_xyz(3.0, 5.0, 3.0),
            ..default()
        })
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(shooting::Shootable)
        .insert(RigidBody::Dynamic)
        .insert(interactions::Interactable(
            interactions::InteractionType::Pickup,
        ));

    // Scene
    commands.spawn(SceneBundle {
        scene: assets.load("blocks.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, -5.0),
        ..default()
    });

    //Stool
    commands.spawn(SceneBundle {
        scene: asset_cache.stool.clone(),
        transform: Transform::from_xyz(1.0, 0.0, 1.0),
        ..default()
    });

    if let Some(teahouse) = asset_cache.buildings.get("buildings/teahouse.glb#Scene0") {
        commands
            .spawn(SceneBundle {
                scene: teahouse.clone(),
                transform: Transform::from_xyz(10.0, 0.0, 0.0),
                ..default()
            })
            .insert(Name::from("Teahouse"));
    }
}

#[derive(Component)]
pub struct ColliderPrepped;

//TODO: Remove the 2 following systems and fully integrate the blender workflow
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
