use std::fmt::Debug;

struct Monster {
    health: i32,
}

#[derive(Debug)]
struct Wizard {
    health: i32,
}

#[derive(Debug)]
struct Ranger {
    health: i32,
}

// Trait bounds
trait Magic {}
trait FightClose {}
trait FightFromDistance {}

impl FightClose for Ranger {} // Each type gets FightClose
impl FightClose for Wizard {}
impl FightFromDistance for Ranger {} // Only Ranger gets FightFromDistance
impl Magic for Wizard {} // only Wizard gets Magic

fn attack_with_bow<T: FightFromDistance + Debug> (character: &T, opponent: &mut Monster, distance: u32) {
    if distance < 10 {
        opponent.health -= 10;
        println!(
            "You ttack with your bow. You opponent now has {} health left. You are not at : {:?}",
            opponent.health, character
        );
    }
}

fn attack_with_sword<T: FightClose + Debug>(character: &T, opponent: &mut Monster) {
    opponent.health -= 10;
    println!(
        "You attack with your sword. Your opponent now has {} health left. You are now at: {:?}",
        opponent.health, character
    );
}

fn fireball<T: Magic + Debug>(character: &T, opponent: &mut Monster, distance: u32) {
    if distance < 15 {
        opponent.health -= 20;
        println!("You raise your hands and case a fireball! Your opponent now has {} health left. You are now at: {:?}",
            opponent.health, character
        )
    }
}

fn main() {
    let radagast = Wizard { health: 60 };
    let aragorn = Ranger { health: 80 };

    let mut uruk_hai = Monster { health: 40 };

    attack_with_sword(&radagast, &mut uruk_hai);
    attack_with_bow(&aragorn, &mut uruk_hai, 8);
    fireball(&radagast, &mut uruk_hai, 8);
}
