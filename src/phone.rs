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

use crate::hud::{PhoneDisplay, WalletDisplay};

pub struct PhonePlugin;

impl Plugin for PhonePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PhoneScreen>()
            .register_type::<PhoneBody>()
            .add_systems(OnEnter(crate::GameState::RunAndGun), setup_phone)
            .add_systems(
                Update,
                setup_phone_components.run_if(in_state(crate::GameState::RunAndGun)),
            );
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
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut asset_cache: ResMut<crate::AssetCache>,
) {
    let size = Extent3d {
        width: 512,
        height: 256,
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
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::SEA_GREEN),
            },
            camera: Camera {
                order: 4,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        PhoneScreen,
    ));

    // Save the render texture to apply to the screen once the phone has loaded
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    asset_cache.screen_material = material_handle;

    //Phone Camera
    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            camera: Camera {
                order: 3,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -4.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        RenderLayers::layer(1),
    ));

    //Phone Body
    commands.spawn(SceneBundle {
        scene: asset_cache.phone.clone(),
        transform: Transform::from_xyz(2.8, 1.6, 1.4)
            .with_rotation(Quat::from_axis_angle(Vec3::Y, 0.2)),
        ..default()
    });

    // Light: They are currently shared between passes
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(2.8, 1.6, 0.8)),
            point_light: PointLight {
                intensity: 28.5,
                range: 1.5,
                ..default()
            },
            ..default()
        })
        .insert(RenderLayers::layer(1));

    // Phone UI Elements

    // Banner
    // commands.spawn(SpriteBundle {
    //     sprite: Sprite {
    //         color: Color::RED,
    //         custom_size: Some(Vec2::new(800.0, 50.0)),
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(0.0, 0.0, 2.0),
    //     ..default()
    // });

    commands
        .spawn(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection::new(
                        "Dosh: ",
                        TextStyle {
                            font_size: 48.0,
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "$0.00",
                        TextStyle {
                            font_size: 48.0,
                            ..default()
                        },
                    ),
                ],
                alignment: TextAlignment::Center,
                ..default()
            },
            transform: Transform::from_xyz(0.0, -1.2, 0.0),
            ..default()
        })
        .insert(WalletDisplay);

    commands
        .spawn(Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(
                    "",
                    TextStyle {
                        font_size: 48.0,
                        ..default()
                    },
                )],
                alignment: TextAlignment::Center,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 43.5, 0.0),
            ..default()
        })
        .insert(PhoneDisplay);
}

fn setup_phone_components(
    mut commands: Commands,
    asset_cache: Res<crate::AssetCache>,
    screen_query: Query<&Children, With<PhoneScreen>>,
    body_query: Query<&Children, With<PhoneBody>>,
    mut screen_children_query: Query<Entity, (Without<PhoneScreen>, Without<PhoneBody>)>,
    mut render_layers_inserted: Local<u8>,
) {
    if *render_layers_inserted < 2 {
        for children in &screen_query {
            for child in children {
                if let Ok(_) = screen_children_query.get_mut(*child) {
                    commands
                        .entity(*child)
                        .remove::<Handle<StandardMaterial>>()
                        .insert(asset_cache.screen_material.clone());
                }
                commands.entity(*child).insert(RenderLayers::layer(1));
                *render_layers_inserted += 1;
                println!("Added Render Layer to Screen");
            }
        }

        for children in &body_query {
            for child in children {
                commands.entity(*child).insert(RenderLayers::layer(1));
                *render_layers_inserted += 1;
                println!("Added Render Layer to Body");
            }
        }
    }
}
