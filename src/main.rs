#![allow(unused_imports)]
use libsimulation::{
    characters::{Action, MainCharacter},
    npc::{Mood, Person},
    result,
};

fn main() {
    let p = MainCharacter::build("Example Name", 15, Mood::Neutral);
    let mut character = Person::spawn();
    result!(p.action(Action::Beat).select(&mut character));
}
