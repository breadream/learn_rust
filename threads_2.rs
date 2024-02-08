fn main() {
    let mut my_string = String::from("Can I go inside the thread?");

    let handle = std::thread::spawn(move|| {
        println!("{}", my_string);
    });

    //std::mem::drop(my_string); // we can't drop because handle has it.

    // handle.join();
    handle.join().unwrap();
}
