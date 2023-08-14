use crate::hud::WalletDisplay;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

pub struct MoneyPlugin;

impl Plugin for MoneyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Debts::default())
            .insert_resource(Wallet::default())
            .add_systems(
                Update,
                (wallet_tracking, pop_up_movement).run_if(in_state(crate::GameState::RunAndGun)),
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

fn pop_up_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut pop_up_query: Query<(Entity, &mut PopUp, &mut Transform)>,
) {
    for (entity, mut popup, mut transform) in &mut pop_up_query {
        let frequency = 5.0;
        let phase = 15.0;

        let x_offset =
            popup.starting_x + ((time.elapsed_seconds() * frequency + phase).sin()) * 50.0;
        transform.translation -= Vec3::Y * 30.0 * time.delta_seconds();
        transform.translation.x = x_offset;
        popup.tick(time.delta());
        if popup.finished() {
            commands.entity(entity).despawn_recursive();
        }
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
pub struct PopUp {
    timer: Timer,
    starting_x: f32,
}

impl PopUp {
    pub fn new(starting_x: f32) -> Self {
        PopUp {
            timer: Timer::from_seconds(2.0, TimerMode::Once),
            starting_x,
        }
    }

    pub fn tick(&mut self, delta: std::time::Duration) {
        self.timer.tick(delta);
    }

    pub fn finished(&self) -> bool {
        self.timer.finished()
    }
}

pub struct PopUpCommand(pub f32);

impl Command for PopUpCommand {
    fn apply(self, world: &mut World) {
        let popup_color = if self.0 < 0.0 {
            Color::RED
        } else {
            Color::GREEN
        };

        use rand::Rng;
        let mut rng = rand::thread_rng();
        let x_pos = rng.gen_range(-200.0..200.0);

        let display_text = if self.0 < 0.0 {
            format!("{:.2}", self.0)
        } else {
            format!("+{:.2}", self.0)
        };

        world.spawn((
            Sprite {
                color: popup_color,
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: display_text,
                        style: TextStyle {
                            font_size: 30.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    }],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                transform: Transform::from_xyz(x_pos, -10.0, 0.0),
                ..default()
            },
            PopUp::new(x_pos),
        ));
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

    pub fn debit(&mut self, amount: Money) -> PopUpCommand {
        self.funds -= amount;
        PopUpCommand(-amount.0)
    }

    pub fn credit(&mut self, amount: Money) -> PopUpCommand {
        self.funds += amount;
        PopUpCommand(amount.0)
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
