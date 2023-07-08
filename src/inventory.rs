use crate::weapon::Gun;
use bevy::{prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct Inventory {
    items: HashMap<ItemId, Item>,
}

#[derive(Resource)]
pub struct Belt {
    pub gun: Gun,
}

pub struct Item {
    name: String,
    description: String,
    base_value: u16,
    amount: u8,
    item_types: Vec<ItemType>,
}

impl Item {
    pub fn matches_type(&self, item_type: ItemType) -> bool {
        self.item_types.contains(&item_type)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_value(&self, value_modifier: f32) -> u16 {
        (self.base_value as f32 * value_modifier) as u16
    }

    pub fn get_amount(&self) -> u8 {
        self.amount
    }

    pub fn consume(&mut self, desired_amount: u8) -> u8 {
        let amount_consumed = if desired_amount > self.amount {
            self.amount
        } else {
            desired_amount
        };
        self.amount -= amount_consumed;
        amount_consumed
    }
}

pub struct ItemId(u16);

#[derive(PartialEq)]
pub enum ItemType {
    Crafting,
    Consumable,
    Important,
    Key,
    Throwable,
}
