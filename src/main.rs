use libsimulation::npc::Person;

fn main() {
    let spawn = Person::spawn();
    println!("{:#?}", spawn);
}
