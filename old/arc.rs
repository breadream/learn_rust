use std::sync::{Arc, Mutex};
use std::thread::spawn;

fn make_arc(number: i32) -> Arc<Mutex<i32>> {
    Arc::new(Mutex::new(number))
}

fn new_clone(input: &Arc<Mutex<i32>>) -> Arc<Mutex<i32>> {
    Arc::clone(&input)
}

fn main() {
    let mut handle_vec = vec![];
    let my_num = make_arc(0);

    for _ in 0..2 {
        let _my_num_clone = new_clone(&my_num);
        let handle = spawn(move || {
            for _ in 0..10 {
                let mut value_inside = _my_num_clone.lock().unwrap();
                *value_inside += 1;
            }
        });
        handle_vec.push(handle);
    }

    handle_vec.into_iter().for_each(|handle| handle.join().unwrap());
    
    println!("{:?}", my_num);
}
