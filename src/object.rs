pub use bevy::prelude::*;

#[derive(Component)]
pub struct Moveable;

#[derive(Component)]
pub struct Activatable;

#[derive(Component)]
pub struct Collidable;

#[derive(Component)]
pub struct Interatable(pub InteractionType);

pub enum InteractionType {
    Money(u16),
    Item,
    Activate,
}
