use bevy::prelude::*;

pub struct CollisionEvent {
    pub body_a: CollidedBody,
    pub body_b: CollidedBody,
}

pub struct CollidedBody {
    pub entity: Entity,
    pub physics_data: CollisionData,
}

pub struct CollisionData {
    mass: f32,
    direction: Vec3,
    speed: f32,
    damage_multiplier: f32,
    force_threshold: f32,
    force_dampener: f32,
}
