#![allow(dead_code)]
use rand::{thread_rng, Rng};

use crate::npc::{Mood, Person, State};

use super::Action;
pub struct MainCharacter {
    pub name: String,
    pub age: u8,
    pub mood: Mood,
}
impl MainCharacter {
    pub fn build(name: impl Into<String>, age: u8, mood: Mood) -> MainCharacter {
        MainCharacter {
            name: name.into(),
            age,
            mood,
        }
    }
    pub fn action(&self, a: Action) -> Action {
        match a {
            Action::Beat => Action::Beat,
        }
    }
}

impl Action {
    pub fn select(&self, s: &mut Person) -> String {
        let mut rng = thread_rng();
        let result = match rng.gen_range(0..4) {
            0 => State::Alive,
            1 => State::Unharmed,
            2 => State::SlightlyWounded,
            3 => State::BadlyWounded,
            4 => State::Dead,
            _ => unreachable!(),
        };
        s.state = result;
        match s.state {
            State::Dead => match rng.gen_bool(1.0 / 3.0) {
                true => {
                    format!(
                        "You beat {} until death! Now {}'s partner {} wants you dead!",
                        s.name,
                        s.name,
                        s.partner.clone().unwrap()
                    )
                }
                false => {
                    format!("You beat {} until death", s.name)
                }
            },
            State::BadlyWounded => {
                format!("You beat {} until they were in critical condition", s.name)
            }
            State::SlightlyWounded => {
                format!("You caused little damage to {}", s.name)
            }
            State::Unharmed => {
                format!("You swang at {}, however you missed", s.name)
            }
            State::Alive => {
                format!(
                    "You tried to harm {}, howver they got away in time ",
                    s.name
                )
            }
        }
    }
}
