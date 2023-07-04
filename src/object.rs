pub use bevy::prelude::*;

#[derive(Component)]
pub struct Moveable;

#[derive(Component)]
pub struct Activatable;

#[derive(Component)]
pub struct Collidable;

#[derive(Component)]
pub struct Interatable(pub InteractionType);

pub struct ItemId(u16);

pub enum InteractionType {
    Money(u32),
    Item(ItemId),
    Activate,
}

#[derive(Resource)]
pub struct Debts {
    medical: u32,
    rent: u32,
    utilities: u32,
}

#[derive(Resource)]
pub struct Wallet {
    funds: f32,
}
