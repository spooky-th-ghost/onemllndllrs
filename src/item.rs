use crate::inventory::Belt;
use bevy::prelude::*;

pub trait Item {
    fn add(&self, belt: &ResMut<Belt>);
    fn remove(&self, belt: &ResMut<Belt>);
    fn item_type(&self) -> ItemType;
}

pub struct ItemId(pub u16);

#[derive(Clone, Copy, Debug)]
pub enum ItemType {
    KeyItem,
    Clothes,
    Tech,
    Part,
    HeldItem,
    Consumable,
    Food,
    Drink,
    Patch,
}
