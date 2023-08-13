use crate::hud::WalletDisplay;
use bevy::prelude::*;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

pub struct MoneyPlugin;

impl Plugin for MoneyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Debts::default())
            .insert_resource(Wallet::default())
            .add_systems(
                Update,
                wallet_tracking.run_if(in_state(crate::GameState::RunAndGun)),
            );
    }
}

fn wallet_tracking(
    wallet: Res<Wallet>,
    mut wallet_display_query: Query<&mut Text, With<WalletDisplay>>,
) {
    for mut text in &mut wallet_display_query {
        text.sections[1].value = wallet.funds.to_string();
    }
}

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

impl Default for Debts {
    fn default() -> Self {
        Debts {
            medical: 1000000.0.into(),
            rent: 0.0.into(),
            utilities: 0.0.into(),
        }
    }
}

#[derive(Component)]
pub struct PopUp;

#[derive(Bundle)]
pub struct PopupBundle {
    text_2d_bundle: Text2dBundle,
    sprite: Sprite,
    pop_up: PopUp,
}

impl PopupBundle {
    pub fn new(amount: f32) -> Self {
        let popup_color = if amount < 0.0 {
            Color::RED
        } else {
            Color::GREEN
        };

        PopupBundle {
            sprite: Sprite {
                color: popup_color,
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            text_2d_bundle: Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: amount.to_string(),
                        style: TextStyle {
                            font_size: 30.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    }],
                    alignment: TextAlignment::Center,
                    //TODO: Spawn at a randomized x location around the money display
                    ..default()
                },
                ..default()
            },
            pop_up: PopUp,
        }
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

impl Default for Wallet {
    fn default() -> Self {
        Wallet {
            funds: 25.30.into(),
        }
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

impl Mul<f32> for Money {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Money(self.0 * rhs)
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:.2}", self.0)
    }
}
