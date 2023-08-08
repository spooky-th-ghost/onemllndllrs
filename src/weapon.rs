use crate::{
    camera::CameraFocus,
    money::{Money, Wallet},
};
use bevy::prelude::*;
use std::time::Duration;

pub struct Gun {
    muzzle: Muzzle,
    receiver: Receiver,
    clip: Clip,
    trigger: Trigger,
    reload_timer: Timer,
    reloading: bool,
}

impl Default for Gun {
    fn default() -> Self {
        Gun {
            muzzle: Muzzle {
                min_spread: 0.0,
                max_spread: 30.0,
                current_spread: 0.0,
                bloom: 1.0,
                max_range: 30.0,
            },
            receiver: Receiver {
                fire_type: FireType::Hitscan,
                base_damage: 10,
                force_transfer: 1.0,
                kick: 2.0,
            },
            clip: Clip::default(),
            trigger: Trigger::auto(),
            reload_timer: Timer::default(),
            reloading: false,
        }
    }
}

impl Gun {
    pub fn tick(&mut self, delta: Duration) {
        self.trigger.tick(delta);
        self.reload_timer.tick(delta);
        self.muzzle.reduce_spread();
        if self.reload_timer.finished() {
            self.reloading = false;
        }
    }

    pub fn fire(&mut self, camera_focus: Res<CameraFocus>) -> FireResult {
        if self.trigger.can_fire() && !self.reloading {
            if self.clip.spend_ammo() {
                use FireType::*;
                let shot = match self.receiver.fire_type {
                    Hitscan => ShotEvent::Raycast(vec![RaycastShot {
                        base_damage: self.receiver.base_damage,
                        range: self.muzzle.get_range(),
                        force: self.receiver.get_force(),
                        dir: camera_focus.forward_randomized(self.muzzle.get_spread()),
                        origin: camera_focus.origin(),
                    }]),
                    HitscanSpread(amount) => {
                        let mut shots_vec: Vec<RaycastShot> = Vec::new();

                        for _ in 1..amount {
                            shots_vec.push(RaycastShot {
                                base_damage: self.receiver.base_damage,
                                range: self.muzzle.get_range(),
                                force: self.receiver.get_force(),
                                dir: camera_focus.forward_randomized(self.muzzle.get_spread()),
                                origin: camera_focus.origin(),
                            });
                        }
                        ShotEvent::Raycast(shots_vec)
                    }
                    Projectile => ShotEvent::Projectile(vec![ProjectileShot {
                        base_damage: self.receiver.base_damage,
                        speed: 30.0,
                        force: self.receiver.get_force(),
                        dir: camera_focus.forward_randomized(self.muzzle.get_spread()),
                        origin: camera_focus.origin(),
                    }]),
                    ProjectileSpread(amount) => {
                        let mut shots_vec: Vec<ProjectileShot> = Vec::new();

                        for _ in 1..amount {
                            shots_vec.push(ProjectileShot {
                                base_damage: self.receiver.base_damage,
                                speed: 30.0,
                                force: self.receiver.get_force(),
                                dir: camera_focus.forward_randomized(self.muzzle.get_spread()),
                                origin: camera_focus.origin(),
                            });
                        }
                        ShotEvent::Projectile(shots_vec)
                    }
                };
                self.trigger.fire();
                self.muzzle.increase_spread();
                FireResult::Shot(shot)
            } else {
                FireResult::EmptyClip
            }
        } else {
            FireResult::NoAction
        }
    }

    pub fn current_ammo(&self) -> u8 {
        self.clip.current
    }

    pub fn reload(&mut self, mut wallet: ResMut<Wallet>) {
        let percentage_purchased = self.clip.reload();

        if percentage_purchased > 0.0 {
            wallet.debit(Money::from(percentage_purchased));
            self.reload_timer = Timer::from_seconds(self.clip.get_reload_time(), TimerMode::Once);
        }
    }

    pub fn is_reloading(&self) -> bool {
        self.reloading
    }

    pub fn get_trigger_mode(&self) -> TriggerMode {
        self.trigger.get_trigger_mode()
    }
}

pub enum Shot {
    SingleHitscan {
        base_damage: u16,
        range: f32,
        force_applied: f32,
    },
    MultiHitscan {
        base_damage: u16,
        range: f32,
        force_applied: f32,
        count: u8,
        spread: f32,
    },
    SingleProjectile {
        base_damage: u16,
        range: f32,
        force_applied: f32,
    },
    MultiProjectile {
        base_damage: u16,
        range: f32,
        force_applied: f32,
        count: u8,
        spread: f32,
    },
}

pub enum FireResult {
    Shot(ShotEvent),
    EmptyClip,
    NoAction,
}

#[derive(Event)]
pub enum ShotEvent {
    Raycast(Vec<RaycastShot>),
    Projectile(Vec<ProjectileShot>),
}

#[derive(Default)]
pub struct RaycastShot {
    pub base_damage: u16,
    pub origin: Vec3,
    pub dir: Vec3,
    pub range: f32,
    pub force: f32,
}

#[derive(Default)]
pub struct ProjectileShot {
    pub base_damage: u16,
    pub origin: Vec3,
    pub dir: Vec3,
    pub speed: f32,
    pub force: f32,
}

#[derive(Default)]
pub struct Receiver {
    fire_type: FireType,
    base_damage: u16,
    force_transfer: f32,
    kick: f32,
}

impl Receiver {
    fn get_force(&self) -> f32 {
        self.force_transfer
    }
}

pub struct Clip {
    max: u8,
    current: u8,
    reload_time: f32,
    clip_cost: Money,
}

impl Clip {
    fn get_clip_stats(&self) -> ClipStats {
        ClipStats {
            max: self.max,
            current: self.current,
        }
    }

    fn get_reload_time(&self) -> f32 {
        self.reload_time
    }

    fn spend_ammo(&mut self) -> bool {
        if self.current > 0 {
            self.current -= 1;
            true
        } else {
            false
        }
    }

    fn reload(&mut self) -> f32 {
        if self.max == self.current {
            0.0
        } else {
            let difference = self.max - self.current;
            self.current = self.max;
            1.0 - (difference as f32 / self.max as f32)
        }
    }
}

impl Default for Clip {
    fn default() -> Self {
        Clip {
            current: 30,
            max: 30,
            reload_time: 1.0,
            clip_cost: 10.0.into(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct ClipStats {
    pub max: u8,
    pub current: u8,
}

#[derive(Default)]
pub struct Muzzle {
    min_spread: f32,
    max_spread: f32,
    current_spread: f32,
    bloom: f32,
    max_range: f32,
}

impl Muzzle {
    fn get_spread(&self) -> f32 {
        self.current_spread
    }

    fn get_range(&self) -> f32 {
        self.max_range
    }

    fn increase_spread(&mut self) {
        let start = self.current_spread;
        let end = self.max_spread;
        let t = self.bloom;
        self.current_spread = start * (1.0 - t) + (end * t);
    }

    fn reduce_spread(&mut self) {
        let start = self.current_spread;
        let end = self.min_spread;
        let t = self.bloom * 0.25;
        self.current_spread = start * (1.0 - t) + (end * t);
    }
}

#[derive(Default)]
pub struct Trigger {
    trigger_mode: TriggerMode,
    shot_timer: Timer,
    pullable: bool,
}

impl Trigger {
    pub fn auto() -> Self {
        Trigger {
            trigger_mode: TriggerMode::Auto,
            shot_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            ..default()
        }
    }

    pub fn semi_auto() -> Self {
        Trigger {
            trigger_mode: TriggerMode::SemiAuto,
            shot_timer: Timer::from_seconds(0.3, TimerMode::Repeating),
            ..default()
        }
    }

    fn can_fire(&self) -> bool {
        self.pullable
    }

    fn tick(&mut self, delta: Duration) {
        if !self.pullable {
            self.shot_timer.tick(delta);
            if self.shot_timer.finished() {
                self.pullable = true;
                self.shot_timer.reset();
            }
        }
    }

    fn get_trigger_mode(&self) -> TriggerMode {
        self.trigger_mode
    }

    fn fire(&mut self) {
        self.pullable = false;
    }
}

#[derive(Clone, Copy, Default)]
pub enum TriggerMode {
    #[default]
    Auto,
    SemiAuto,
}

#[derive(Default)]
pub enum FireType {
    #[default]
    Hitscan,
    HitscanSpread(u8),
    Projectile,
    ProjectileSpread(u8),
}
