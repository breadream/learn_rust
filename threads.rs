fn main() {
    for _ in 0..10 {
        std::thread::spawn(|| {
            println!("I am printing something");
        });
    }
}
