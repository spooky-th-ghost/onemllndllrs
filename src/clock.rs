use crate::hud::PhoneDisplay;
use bevy::prelude::*;
use std::ops::{Add, AddAssign};

pub struct ClockPlugin;

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_clock)
            .add_systems(Update, (advance_time, display_time));
    }
}

#[derive(Copy, Clone, Default, PartialEq, PartialOrd)]
pub enum Day {
    #[default]
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl Day {
    fn index(&self) -> u8 {
        *self as u8
    }
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_representation = match self {
            Self::Sunday => "Sunday",
            Self::Monday => "Monday",
            Self::Tuesday => "Tuesday",
            Self::Wednesday => "Wednesday",
            Self::Thursday => "Thursday",
            Self::Friday => "Friday",
            Self::Saturday => "Saturday",
        };
        write!(f, "{}", string_representation)
    }
}

impl From<u8> for Day {
    fn from(value: u8) -> Self {
        let index = value % 7;
        match index {
            0 => Self::Sunday,
            1 => Self::Monday,
            2 => Self::Tuesday,
            3 => Self::Wednesday,
            4 => Self::Thursday,
            5 => Self::Friday,
            _ => Self::Saturday,
        }
    }
}

impl Add<u8> for Day {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        let new_index = (self.index() + rhs) % 7;
        new_index.into()
    }
}

impl AddAssign<u8> for Day {
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs;
    }
}

#[derive(Copy, Clone, Default, PartialEq, PartialOrd)]
pub enum Minute {
    #[default]
    Flat,
    Quarter,
    Half,
    FortyFive,
}

impl Minute {
    fn index(&self) -> u8 {
        *self as u8
    }
}

impl std::fmt::Display for Minute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_representation = match self {
            Self::Flat => "00",
            Self::Quarter => "15",
            Self::Half => "30",
            Self::FortyFive => "45",
        };

        write!(f, "{}", string_representation)
    }
}

impl From<u8> for Minute {
    fn from(value: u8) -> Self {
        let index = value % 4;
        match index {
            0 => Self::Flat,
            1 => Self::Quarter,
            2 => Self::Half,
            _ => Self::FortyFive,
        }
    }
}

impl Add<u8> for Minute {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        let new_index = (self.index() + rhs) % 4;
        new_index.into()
    }
}

impl AddAssign<u8> for Minute {
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs;
    }
}

#[derive(Resource, Default)]
pub struct Phone {
    pub date: Date,
    pub timer: Timer,
}

impl Phone {
    pub fn tick(&mut self, delta: std::time::Duration) {
        self.timer.tick(delta);
        if self.timer.just_finished() {
            self.date.advance();
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Date {
    pub minute: Minute,
    pub hour: u8,
    pub day: Day,
}

impl Date {
    pub fn new(day: Day, hour: u8, minute: Minute) -> Self {
        Date { day, hour, minute }
    }

    pub fn advance(&mut self) -> bool {
        let old_date = self.clone();
        self.minute += 1;
        if self.minute.index() < old_date.minute.index() {
            self.hour = (self.hour + 1) % 24;
            if self.hour < old_date.hour {
                self.day += 1;
                return true;
            }
        }
        false
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {:0>2}:{}",
            self.day.to_string(),
            self.hour,
            self.minute
        )
    }
}

fn spawn_clock(mut commands: Commands) {
    let phone = Phone {
        date: Date::new(Day::Monday, 18, Minute::Half),
        timer: Timer::from_seconds(60.0, TimerMode::Repeating),
    };

    println!("{}", phone.date.to_string());

    commands.insert_resource(phone);
}

pub fn advance_time(time: Res<Time>, mut phone: ResMut<Phone>) {
    phone.tick(time.delta());
}

pub fn display_time(
    phone: Res<Phone>,
    mut phone_display_query: Query<&mut Text, With<PhoneDisplay>>,
) {
    for mut text in &mut phone_display_query {
        text.sections[0].value = phone.date.to_string();
    }
}
