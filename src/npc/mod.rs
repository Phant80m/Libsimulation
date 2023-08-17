use serde::{Deserialize, Serialize};
mod people;
#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub mood: Mood,
    pub partner: Option<String>,
    pub kids: Vec<Person>,
}
#[derive(Default, Debug, Deserialize, Serialize)]
pub enum Mood {
    Happy,
    #[default]
    Neutral,
    Sad,
    Angry,
}
