use bevy::prelude::*;

pub struct Gun {
    muzzle: Muzzle,
    receiver: Receiver,
    clip: Clip,
    trigger: Trigger,
}

impl Gun {
    pub fn fire(&mut self) {
        if self.clip.spend_ammo() {
            //TODO: Create Shot
            use FireType::*;
            match self.receiver.fire_type {
                Hitscan => {}
                Projectile => {}
                ProjectileSpread => {}
            }
        } else {
            //TODO: Play the click sound
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
    reload_speed: f32,
}

impl Clip {
    fn get_max(&self) -> u8 {
        self.max
    }

    fn get_current(&self) -> u8 {
        self.current
    }

    fn spend_ammo(&mut self) -> bool {
        if self.current > 0 {
            self.current -= 1;
            true
        } else {
            false
        }
    }

    fn reload(&mut self) {
        self.current = self.max;
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

pub enum Trigger {
    Auto,
    SemiAuto,
}

pub enum FireType {
    Hitscan,
    Projectile,
    ProjectileSpread,
}
