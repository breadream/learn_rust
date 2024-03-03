// Import the rand crate
extern crate rand;

use rand::Rng; // Import the trait Rng from the rand crate

fn main() {
    let mut rng = rand::thread_rng(); // Create a random number generator

    for _ in 0..5 {
        let random_u16 = rng.gen::<u16>(); // Generate a random u16 number
        print!("{} ", random_u16);
    }
}
