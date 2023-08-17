pub mod characters;
pub mod generator;
pub mod npc;

#[macro_export]
macro_rules! result {
    ($expr:expr) => {{
        let result = $expr;
        println!("{:#?}", result);
        result
    }};
}
