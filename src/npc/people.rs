use crate::{generator::Name, npc::Mood, npc::Person};
use rand::{thread_rng, Rng};
impl Person {
    pub fn new(
        name: impl Into<String>,
        age: impl Into<u8>,
        mood: Mood,
        partner: Option<String>,
        kids: Vec<Person>,
    ) -> Self {
        Self {
            name: name.into(),
            age: age.into(),
            mood,
            partner,
            kids,
        }
    }
    pub fn spawn() -> Self {
        let mut rng = thread_rng();
        let is_male: bool = rng.gen_bool(1.0 / 2.0);
        let name = match is_male {
            true => Name::gen_male(),
            false => Name::gen_female(),
        };

        let full_name = format!("{} {}", name.0, name.1);
        let age: u8 = rng.gen_range(0..105);
        let mood: Mood = match rng.gen_range(0..4) {
            0 => Mood::Happy,
            1 => Mood::Neutral,
            2 => Mood::Sad,
            3 => Mood::Angry,
            _ => unreachable!(),
        };
        let mut kids = Vec::new();
        if age > 30 {
            let num_kids: usize = rng.gen_range(0..5);
            for _ in 0..num_kids {
                kids.push(Self::spawn_child(name.1.clone()));
            }
        }
        let partner_gen = match name.2 {
            true => Name::gen_female(),
            false => Name::gen_male(),
        };
        let mut partner: Option<String> = None;
        if age >= 22 {
            partner = Some(format!("{} {}", partner_gen.0, name.1));
        }

        Self {
            name: full_name,
            age,
            mood,
            partner,
            kids,
        }
    }
    pub fn spawn_child(last_name: String) -> Self {
        let mut rng = thread_rng();
        let name = format!("{} {}", Name::gen_first_last().0, last_name);
        let age: u8 = rng.gen_range(0..19);
        let mood: Mood = match rng.gen_range(0..4) {
            0 => Mood::Happy,
            1 => Mood::Neutral,
            2 => Mood::Sad,
            3 => Mood::Angry,
            _ => unreachable!(),
        };

        Self {
            name,
            age,
            mood,
            partner: None,
            kids: Vec::new(),
        }
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    pub fn to_toml(&self) -> String {
        toml::to_string(&self).unwrap()
    }
}
