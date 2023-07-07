use bevy::prelude::*;

pub struct Gun {
    receiver: Receiver,
    clip: Clip,
    spread: Spread,
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

pub struct Spread {
    min_spread: f32,
    max_spread: f32,
    bloom: f32,
}

pub enum FireType {
    Hitscan,
    Projectile,
}
