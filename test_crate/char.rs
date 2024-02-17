use std::convert::TryFrom;
use rand::prelude::*;

fn main() {
    let some_char = char::from(99);
    println!("{}", some_char);

    let mut random_gen = rand::thread_rng();
    for _ in 0..40_000 {
        let bigger_character = char::try_from(random_gen.gen_range(std::u32::MIN..std::u32::MAX)).unwrap_or('-');
        print!("{}", bigger_character)
    }
}
