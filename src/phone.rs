use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};

pub struct PhonePlugin;

impl Plugin for PhonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::GameState::RunAndGun), setup_phone);
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PhoneScreen;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PhoneBody;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PhoneScreenCamera;

fn setup_phone(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    assets: Res<AssetServer>,
) {
    let size = Extent3d {
        width: 128,
        height: 64,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    //Phone Screen Camera
    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::SEA_GREEN),
                ..default()
            },
            camera: Camera {
                // Render before every other camera
                order: -2,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(2),
        PhoneScreen,
    ));

    //Phone Camera
    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            camera: Camera {
                // Render after the screen camera, and before the main pass
                order: -1,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        RenderLayers::layer(1),
    ));

    //Phone Body
    commands.spawn(SceneBundle {
        scene: assets.load("phone.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, -2.0),
        ..default()
    });

    // Light: They are currently shared between passes
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });
}
