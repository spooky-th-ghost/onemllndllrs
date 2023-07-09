use bevy::prelude::*;

#[derive(Resource)]
pub struct Debts {
    medical: f32,
    rent: f32,
    utilities: f32,
}

impl Debts {
    pub fn total(&self) -> f32 {
        self.medical + self.rent + self.utilities
    }
}

#[derive(Resource)]
pub struct Wallet {
    funds: f32,
}

impl Wallet {
    pub fn debit(&mut self, amount: f32) {
        self.funds -= amount;
    }

    pub fn credit(&mut self, amount: f32) {
        self.funds += amount;
    }
}
