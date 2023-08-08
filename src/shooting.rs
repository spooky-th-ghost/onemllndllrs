use bevy::prelude::*;
use bevy_rapier3d::prelude::{ExternalImpulse, RapierContext, RigidBody};
use leafwing_input_manager::prelude::*;

use crate::audio::{EmptySound, SoundBank};
use crate::camera::CameraFocus;
use crate::hud::AmmoDisplay;
use crate::inventory::Belt;
use crate::money::Wallet;
use crate::weapon::{FireResult, ShotEvent, TriggerMode};
use crate::{input::PlayerAction, movement::Player, GameState, PlayerSet};

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShotEvent>()
            .insert_resource(Belt::default())
            .configure_set(
                Update,
                PlayerSet::Combat.run_if(in_state(GameState::RunAndGun)),
            )
            .add_systems(
                Update,
                (
                    send_shot_events,
                    read_shot_events,
                    render_bulletholes,
                    gun_upkeep,
                    track_ammo,
                    rotate_clip,
                    reload_gun,
                )
                    .in_set(PlayerSet::Combat),
            );
    }
}

pub fn gun_upkeep(time: Res<Time>, mut belt: ResMut<Belt>) {
    belt.gun.tick(time.delta());
}

pub fn track_ammo(mut display_query: Query<&mut Text, With<AmmoDisplay>>, belt: Res<Belt>) {
    for mut text in &mut display_query {
        text.sections[0].value = belt.gun.current_ammo().to_string();
    }
}

#[derive(Component)]
pub struct ClipComponent;

fn rotate_clip(
    time: Res<Time>,
    belt: Res<Belt>,
    mut clip_query: Query<&mut Transform, With<ClipComponent>>,
) {
    for mut transform in &mut clip_query {
        let rotation_speed = if belt.gun.is_reloading() {
            50.0_f32.to_radians()
        } else {
            25.0_f32.to_radians()
        };
        transform.rotate_y(25.0_f32.to_radians() * time.delta_seconds());
    }
}

fn reload_gun(
    mut belt: ResMut<Belt>,
    mut wallet: ResMut<Wallet>,
    player_query: Query<&ActionState<PlayerAction>>,
) {
    if let Ok(action) = player_query.get_single() {
        if !belt.gun.is_reloading() && action.just_pressed(PlayerAction::Reload) {
            belt.gun.reload(wallet);
        }
    }
}

pub fn send_shot_events(
    mut commands: Commands,
    mut player_query: Query<&ActionState<PlayerAction>, With<Player>>,
    empty_query: Query<Entity, (With<EmptySound>, Without<Player>)>,
    camera_focus: Res<CameraFocus>,
    sound_bank: Res<SoundBank>,
    mut belt: ResMut<Belt>,
    mut shot_events: EventWriter<ShotEvent>,
) {
    //TODO: Next steps
    // 3. ammo count
    // 4. reloading
    // 5. money
    let action = player_query.single_mut();

    let shot_to_fire = match belt.get_trigger_mode() {
        TriggerMode::Auto => {
            if action.pressed(PlayerAction::Shoot) {
                belt.fire(camera_focus)
            } else {
                FireResult::NoAction
            }
        }
        TriggerMode::SemiAuto => {
            if action.just_pressed(PlayerAction::Shoot) {
                belt.fire(camera_focus)
            } else {
                FireResult::NoAction
            }
        }
    };
    match shot_to_fire {
        FireResult::Shot(shot) => shot_events.send(shot),
        FireResult::EmptyClip => {
            if empty_query.is_empty() {
                commands.spawn(sound_bank.empty_fire());
            }
        }
        _ => (),
    }
}

#[derive(Component)]
pub struct Shootable;

fn read_shot_events(
    mut commands: Commands,
    mut shot_events: EventReader<ShotEvent>,
    player_query: Query<Entity, With<Player>>,
    shootable_query: Query<
        (&Transform, bevy::ecs::query::Has<ExternalImpulse>),
        (With<RigidBody>, With<Shootable>, Without<Player>),
    >,
    sound_bank: Res<SoundBank>,
    rapier_context: Res<RapierContext>,
) {
    if let Ok(entity) = player_query.get_single() {
        for shot_event in shot_events.iter() {
            commands.spawn(sound_bank.bullet_shot());
            match shot_event {
                ShotEvent::Raycast(shots) => {
                    for shot in shots {
                        let ray_origin = shot.origin;
                        let ray_dir = shot.dir;
                        let max_distance = shot.range;
                        let solid = false;
                        let filter = bevy_rapier3d::pipeline::QueryFilter {
                            exclude_collider: Some(entity),
                            exclude_rigid_body: Some(entity),
                            ..default()
                        };

                        if let Some((entity, intersection)) = rapier_context
                            .cast_ray_and_get_normal(
                                ray_origin,
                                ray_dir,
                                max_distance,
                                solid,
                                filter,
                            )
                        {
                            //Bullet Hole
                            commands.spawn((
                                TransformBundle::from_transform(
                                    Transform::default().with_translation(intersection.point),
                                ),
                                BulletHole,
                                Name::new("Hole"),
                            ));
                            if let Ok((hit_transform, has_external)) = shootable_query.get(entity) {
                                let center_of_mass = hit_transform.translation;
                                let impulse = ExternalImpulse::at_point(
                                    shot.dir * 10.0,
                                    intersection.point,
                                    center_of_mass,
                                );

                                if has_external {
                                    commands
                                        .entity(entity)
                                        .remove::<ExternalImpulse>()
                                        .insert(impulse);
                                } else {
                                    commands.entity(entity).insert(impulse);
                                }
                            }
                        }
                    }
                }
                ShotEvent::Projectile(shots) => {}
            }
        }
    }
}

#[derive(Component)]
pub struct BulletHole;

pub fn debug_shooting(
    mut commands: Commands,
    player_query: Query<(Entity, &ActionState<PlayerAction>), With<Player>>,
    camera_focus: Res<CameraFocus>,
    sound_bank: Res<SoundBank>,
    cube_query: Query<
        (&Transform, bevy::ecs::query::Has<ExternalImpulse>),
        (With<RigidBody>, Without<Player>),
    >,
    rapier_context: Res<RapierContext>,
) {
    let (player_entity, action) = player_query.single();

    if action.pressed(PlayerAction::Shoot) {
        commands.spawn(sound_bank.empty_fire());
        // commands.spawn(sound_bank.bullet_shot());
        let ray_origin = camera_focus.origin();
        let ray_dir = camera_focus.forward_randomized(20.0);
        let max_toi = 100.0;
        let solid = false;
        let filter = bevy_rapier3d::pipeline::QueryFilter {
            exclude_collider: Some(player_entity),
            exclude_rigid_body: Some(player_entity),
            ..default()
        };

        if let Some((entity, intersection)) =
            rapier_context.cast_ray_and_get_normal(ray_origin, ray_dir, max_toi, solid, filter)
        {
            let (hit_transform, has_external) = cube_query.get(entity).unwrap();

            let center_of_mass = hit_transform.translation;

            // shot_events.send(ShotEvent {
            //     shot_entity: entity,
            //     shot_impulse: camera_focus.forward() * 10.0,
            //     collision_point: intersection.point,
            // });

            let impulse = ExternalImpulse::at_point(
                camera_focus.forward() * 10.0,
                intersection.point,
                center_of_mass,
            );

            if has_external {
                commands
                    .entity(entity)
                    .remove::<ExternalImpulse>()
                    .insert(impulse);
            } else {
                commands.entity(entity).insert(impulse);
            }
            commands.spawn((
                TransformBundle::from_transform(
                    Transform::default().with_translation(intersection.point),
                ),
                BulletHole,
            ));
        }
    }
}

pub fn render_bulletholes(hole_query: Query<&Transform, With<BulletHole>>, mut gizmos: Gizmos) {
    for transform in &hole_query {
        gizmos.sphere(transform.translation, Quat::default(), 0.1, Color::RED);
    }
}
