use bevy::{prelude::*, utils::hashbrown::hash_set::Difference};
use std::time::Duration;

pub struct Gun {
    muzzle: Muzzle,
    receiver: Receiver,
    clip: Clip,
    trigger: Trigger,
    reload_timer: Timer,
    reloading: bool,
}

impl Gun {
    pub fn tick(&mut self, delta: Duration) {
        self.trigger.tick(delta);
        self.reload_timer.tick(delta);
        if self.reload_timer.finished() {
            self.reloading = false;
        }
    }

    pub fn fire(&mut self) -> Option<Shot> {
        if self.trigger.can_fire() && !self.reloading {
            if self.clip.spend_ammo() {
                //TODO: Create Shot
                use FireType::*;
                let shot = match self.receiver.fire_type {
                    Hitscan => Shot::Hitscan {
                        base_damage: self.receiver.base_damage,
                    },
                    Projectile => Shot::SingleProjectile {
                        base_damage: self.receiver.base_damage,
                    },
                    ProjectileSpread(amount) => Shot::MultiProjectile {
                        base_damage: self.receiver.base_damage,
                        count: amount,
                        spread: 20.0,
                    },
                };
                Some(shot)
            } else {
                //TODO: Play the click sound
                None
            }
        }
    }

    pub fn reload(&mut self) {
        let percentage_purchased = self.clip.reload();

        if percentage_purchased > 0.0 {
            //TODO: Deduct funds
            self.reload_timer = Timer::from_seconds(self.clip.get_reload_time(), TimerMode::Once);
        }
    }
}

pub enum Shot {
    Hitscan {
        base_damage: u16,
    },
    SingleProjectile {
        base_damage: u16,
    },
    MultiProjectile {
        base_damage: u16,
        count: u8,
        spread: f32,
    },
}

pub struct Receiver {
    fire_type: FireType,
    base_damage: u16,
    force_transfer: f32,
}

pub struct Clip {
    max: u8,
    current: u8,
    reload_time: f32,
}

impl Clip {
    fn get_max(&self) -> u8 {
        self.max
    }

    fn get_current(&self) -> u8 {
        self.current
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
        let t = self.bloom;
        self.current_spread = start * (1.0 - t) + (end * t);
    }
}

pub struct Trigger {
    trigger_mode: TriggerMode,
    shot_timer: Timer,
    pullable: bool,
}

impl Trigger {
    fn can_fire() -> bool {
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

    fn fire(&mut self) {
        self.can_fire = false;
    }
}

pub enum TriggerMode {
    Auto,
    SemiAuto,
}

pub enum FireType {
    Hitscan,
    Projectile,
    ProjectileSpread(u8),
}
