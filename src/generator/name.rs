use super::Name;
use rand::{seq::SliceRandom, thread_rng};

impl Name {
    pub fn generate(&self) -> String {
        let first_names = include_str!("first_names").lines().collect::<Vec<_>>();
        let last_names = include_str!("last_names").lines().collect::<Vec<_>>();
        let mut rng = thread_rng();
        let random_first_name = first_names.choose(&mut rng).unwrap();
        let random_last_name = last_names.choose(&mut rng).unwrap();
        format!("{} {}", random_first_name, random_last_name)
    }
    pub fn gen_first_last() -> (String, String) {
        let first_names = include_str!("first_names").lines().collect::<Vec<_>>();
        let last_names = include_str!("last_names").lines().collect::<Vec<_>>();
        let mut rng = thread_rng();
        let random_first_name = first_names.choose(&mut rng).unwrap();
        let random_last_name = last_names.choose(&mut rng).unwrap();
        (random_first_name.to_string(), random_last_name.to_string())
    }
    pub fn gen_male() -> (String, String, bool) {
        let first_names = include_str!("male_names").lines().collect::<Vec<_>>();
        let last_names = include_str!("last_names").lines().collect::<Vec<_>>();
        let mut rng = thread_rng();
        let random_first_name = first_names.choose(&mut rng).unwrap();
        let random_last_name = last_names.choose(&mut rng).unwrap();
        (
            random_first_name.to_string(),
            random_last_name.to_string(),
            true,
        )
    }
    pub fn gen_female() -> (String, String, bool) {
        let first_names = include_str!("female_names").lines().collect::<Vec<_>>();
        let last_names = include_str!("last_names").lines().collect::<Vec<_>>();
        let mut rng = thread_rng();
        let random_first_name = first_names.choose(&mut rng).unwrap();
        let random_last_name = last_names.choose(&mut rng).unwrap();
        (
            random_first_name.to_string(),
            random_last_name.to_string(),
            false,
        )
    }
}
