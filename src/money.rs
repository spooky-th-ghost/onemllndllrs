use std::ops::{Add, AddAssign, Sub, SubAssign};

use bevy::prelude::*;

#[derive(Resource)]
pub struct Debts {
    medical: Money,
    rent: Money,
    utilities: Money,
}

impl Debts {
    pub fn total(&self) -> Money {
        self.medical + self.rent + self.utilities
    }

    pub fn medical(&self) -> Money {
        self.medical
    }

    pub fn rent(&self) -> Money {
        self.rent
    }

    pub fn utilities(&self) -> Money {
        self.utilities
    }
}

#[derive(Resource)]
pub struct Wallet {
    funds: Money,
}

impl Wallet {
    pub fn funds(&self) -> Money {
        self.funds
    }

    pub fn debit(&mut self, amount: Money) {
        self.funds -= amount;
    }

    pub fn credit(&mut self, amount: Money) {
        self.funds += amount;
    }
}

#[derive(Copy, Clone, Default)]
pub struct Money(f32);

impl From<f32> for Money {
    fn from(value: f32) -> Self {
        Money((value * 100.0).round() / 100.0)
    }
}

impl Add for Money {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Money(self.0 + rhs.0)
    }
}

impl AddAssign for Money {
    fn add_assign(&mut self, rhs: Self) {
        *self = Money(self.0 + rhs.0);
    }
}

impl Sub for Money {
    type Output = Money;

    fn sub(self, rhs: Self) -> Self::Output {
        Money(self.0 - rhs.0)
    }
}

impl SubAssign for Money {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Money(self.0 - rhs.0)
    }
}
